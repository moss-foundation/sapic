import React, { createContext, useCallback, useEffect, useMemo, useReducer, useRef } from "react";
import { toast } from "sonner";

import { createNotificationContent } from "@/lib/ui";
import { CHANNEL as ACTIVITY_BROADCASTER_CHANNEL, ActivityEvent } from "@repo/moss-activity-broadcaster";
import { listen } from "@tauri-apps/api/event";

export const MAX_HISTORY_SIZE = 1000; // Limit number of historical events
export const ONESHOT_CLEANUP_DELAY = 1000;
export const PROGRESS_CLEANUP_DELAY = 1000;
export const DEFAULT_DISPLAY_DURATION = 10;

const createActivityNotification = (title: string, detail?: string, persistent: boolean = false) => {
  const toastId = toast(
    createNotificationContent({
      title,
      description: detail,
      icon: "Info",
      buttonText: "Details",
      onButtonClick: () => {
        alert("Details clicked!");
        toast.dismiss(toastId);
      },
      linkText: "Ignore",
      onLinkClick: () => {
        alert("Ignore clicked!");
        toast.dismiss(toastId);
      },
      onClose: () => toast.dismiss(toastId),
    }),
    { duration: persistent ? Infinity : 2000 }
  );
};

const getEventLocation = (
  event: ActivityEvent,
  activityLocationsRef: React.MutableRefObject<Map<string, string>>
): string => {
  if ("oneshot" in event) return event.oneshot.location;
  if ("start" in event) return event.start.location;

  // Progress and Finish events don't have location, they inherit from their Start event
  const activityId = getActivityId(event);
  if (activityId && activityLocationsRef.current.has(activityId)) {
    return activityLocationsRef.current.get(activityId)!;
  }

  return "window"; // Default fallback
};

// Activity events grouped by location
export interface ActivityEventsContextType {
  // Window location events (for status bar)
  windowEvents: {
    activityEvents: ActivityEvent[];
    activeProgressEvents: Map<string, ActivityEvent[]>;
    oneshotEvents: ActivityEvent[];
    hasActiveEvents: boolean;
    latestEvent: ActivityEvent | null;
    displayQueue: ActivityEvent[];
    getStartTitleForActivity: (activityId: string) => string | null;
  };

  clearEvents: () => void;
}

const getEventId = (event: ActivityEvent): number => {
  if ("oneshot" in event) return event.oneshot.id;
  if ("start" in event) return event.start.id;
  if ("progress" in event) return event.progress.id;
  if ("finish" in event) return event.finish.id;
  return -1;
};

const getActivityId = (event: ActivityEvent): string | null => {
  if ("start" in event) return event.start.activityId;
  if ("progress" in event) return event.progress.activityId;
  if ("finish" in event) return event.finish.activityId;
  return null;
};

const getEventType = (event: ActivityEvent): string => {
  if ("oneshot" in event) return "oneshot";
  if ("start" in event) return "start";
  if ("progress" in event) return "progress";
  if ("finish" in event) return "finish";
  return "unknown";
};

interface ActivityState {
  // All events grouped by location
  windowEvents: {
    activityEvents: ActivityEvent[];
    activeProgressEvents: Map<string, ActivityEvent[]>;
    oneshotEvents: ActivityEvent[];
    activeActivities: Set<string>;
    startTitles: Map<string, string>;
    mostRecentEvent: {
      event: ActivityEvent;
      timestamp: number;
      isOneshot: boolean;
    } | null;
    displayQueue: ActivityEvent[];
  };

  timeoutIds: Set<number>; // Track timeouts for cleanup
}

type ActivityAction =
  | { type: "ADD_WINDOW_EVENT"; payload: ActivityEvent }
  | { type: "REMOVE_WINDOW_ONESHOT"; payload: { id: number } }
  | { type: "FINISH_WINDOW_ACTIVITY"; payload: { activityId: string } }
  | { type: "CLEANUP_WINDOW_ACTIVITY_PROGRESS"; payload: { activityId: string } }
  | {
      type: "SET_WINDOW_MOST_RECENT_EVENT";
      payload: { event: ActivityEvent; timestamp: number; isOneshot: boolean } | null;
    }
  | { type: "DEQUEUE_WINDOW_EVENT" }
  | { type: "CLEAR_ALL" }
  | { type: "ADD_TIMEOUT_ID"; payload: { id: number } }
  | { type: "REMOVE_TIMEOUT_ID"; payload: { id: number } };

function activityRouterReducer(state: ActivityState, action: ActivityAction): ActivityState {
  switch (action.type) {
    case "ADD_WINDOW_EVENT": {
      const event = action.payload;
      const eventType = getEventType(event);
      let newDisplayQueue = [...state.windowEvents.displayQueue];

      // Handle display queue - oneshot events go to the front
      if (eventType === "oneshot") {
        newDisplayQueue = [event, ...newDisplayQueue];
      } else if (eventType !== "finish") {
        newDisplayQueue = [...newDisplayQueue, event];
      }

      // Limit activityEvents history size
      let newActivityEvents = [...state.windowEvents.activityEvents, event];
      if (newActivityEvents.length > MAX_HISTORY_SIZE) {
        newActivityEvents = newActivityEvents.slice(-MAX_HISTORY_SIZE);
      }

      let newOneshotEvents = state.windowEvents.oneshotEvents;
      const newActiveActivities = new Set(state.windowEvents.activeActivities);
      const newStartTitles = new Map(state.windowEvents.startTitles);
      const newActiveProgressEvents = new Map(state.windowEvents.activeProgressEvents);

      if (eventType === "oneshot") {
        newOneshotEvents = [...state.windowEvents.oneshotEvents, event];
      } else {
        const activityId = getActivityId(event);
        if (activityId) {
          if (eventType === "start" && "start" in event) {
            newStartTitles.set(activityId, event.start.title);
            newActiveActivities.add(activityId);
          } else if (eventType === "progress") {
            newActiveActivities.add(activityId);
          }

          const currentEvents = newActiveProgressEvents.get(activityId) || [];
          const updatedEvents = [...currentEvents, event].sort((a, b) => getEventId(a) - getEventId(b));
          newActiveProgressEvents.set(activityId, updatedEvents);
        }
      }

      return {
        ...state,
        windowEvents: {
          ...state.windowEvents,
          activityEvents: newActivityEvents,
          displayQueue: newDisplayQueue,
          oneshotEvents: newOneshotEvents,
          activeActivities: newActiveActivities,
          startTitles: newStartTitles,
          activeProgressEvents: newActiveProgressEvents,
        },
      };
    }

    case "REMOVE_WINDOW_ONESHOT": {
      return {
        ...state,
        windowEvents: {
          ...state.windowEvents,
          oneshotEvents: state.windowEvents.oneshotEvents.filter(
            (e) => !("oneshot" in e) || e.oneshot.id !== action.payload.id
          ),
        },
      };
    }

    case "FINISH_WINDOW_ACTIVITY": {
      const { activityId } = action.payload;
      const newActiveActivities = new Set(state.windowEvents.activeActivities);
      newActiveActivities.delete(activityId);

      const newStartTitles = new Map(state.windowEvents.startTitles);
      newStartTitles.delete(activityId);

      return {
        ...state,
        windowEvents: {
          ...state.windowEvents,
          activeActivities: newActiveActivities,
          startTitles: newStartTitles,
        },
      };
    }

    case "CLEANUP_WINDOW_ACTIVITY_PROGRESS": {
      const { activityId } = action.payload;
      const newActiveProgressEvents = new Map(state.windowEvents.activeProgressEvents);
      newActiveProgressEvents.delete(activityId);

      return {
        ...state,
        windowEvents: {
          ...state.windowEvents,
          activeProgressEvents: newActiveProgressEvents,
        },
      };
    }

    case "SET_WINDOW_MOST_RECENT_EVENT": {
      return {
        ...state,
        windowEvents: {
          ...state.windowEvents,
          mostRecentEvent: action.payload,
        },
      };
    }

    case "DEQUEUE_WINDOW_EVENT": {
      if (state.windowEvents.displayQueue.length === 0) return state;
      return {
        ...state,
        windowEvents: {
          ...state.windowEvents,
          displayQueue: state.windowEvents.displayQueue.slice(1),
        },
      };
    }

    case "CLEAR_ALL": {
      // Clear all timeouts
      state.timeoutIds.forEach((id) => clearTimeout(id));

      return {
        windowEvents: {
          activityEvents: [],
          activeProgressEvents: new Map(),
          oneshotEvents: [],
          activeActivities: new Set(),
          startTitles: new Map(),
          mostRecentEvent: null,
          displayQueue: [],
        },
        timeoutIds: new Set(),
      };
    }

    case "ADD_TIMEOUT_ID": {
      const newTimeoutIds = new Set(state.timeoutIds);
      newTimeoutIds.add(action.payload.id);
      return {
        ...state,
        timeoutIds: newTimeoutIds,
      };
    }

    case "REMOVE_TIMEOUT_ID": {
      const newTimeoutIds = new Set(state.timeoutIds);
      newTimeoutIds.delete(action.payload.id);
      return {
        ...state,
        timeoutIds: newTimeoutIds,
      };
    }

    default:
      return state;
  }
}

export const ActivityRouterContext = createContext<ActivityEventsContextType>({
  windowEvents: {
    activityEvents: [],
    activeProgressEvents: new Map(),
    oneshotEvents: [],
    hasActiveEvents: false,
    latestEvent: null,
    displayQueue: [],
    getStartTitleForActivity: () => null,
  },
  clearEvents: () => {},
});

export const ActivityRouterProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const initialState: ActivityState = {
    windowEvents: {
      activityEvents: [],
      activeProgressEvents: new Map(),
      oneshotEvents: [],
      activeActivities: new Set(),
      startTitles: new Map(),
      mostRecentEvent: null,
      displayQueue: [],
    },
    timeoutIds: new Set(),
  };

  const [state, dispatch] = useReducer(activityRouterReducer, initialState);

  const processingQueueRef = useRef(false);
  const displayDurationRef = useRef(DEFAULT_DISPLAY_DURATION);
  const activityLocationsRef = useRef(new Map<string, string>());

  // Safe setTimeout wrapper that tracks timeout IDs for cleanup
  const safeSetTimeout = useCallback((callback: () => void, delay: number): void => {
    const id = window.setTimeout(() => {
      callback();
      dispatch({ type: "REMOVE_TIMEOUT_ID", payload: { id } });
    }, delay);
    dispatch({ type: "ADD_TIMEOUT_ID", payload: { id } });
  }, []);

  const getStartTitleForActivity = useCallback(
    (activityId: string): string | null => {
      // Try from the map first (most efficient)
      const title = state.windowEvents.startTitles.get(activityId);
      if (title) return title;

      // If not in map, search in the activity events (less efficient)
      const startEvent = state.windowEvents.activityEvents.find(
        (event) => "start" in event && event.start.activityId === activityId
      );

      if (startEvent && "start" in startEvent) {
        return startEvent.start.title;
      }

      return null;
    },
    [state.windowEvents.startTitles, state.windowEvents.activityEvents]
  );

  // Process events from the window display queue
  useEffect(() => {
    if (state.windowEvents.displayQueue.length === 0 || processingQueueRef.current) {
      return;
    }

    processingQueueRef.current = true;
    const nextEvent = state.windowEvents.displayQueue[0];
    const isOneshot = "oneshot" in nextEvent;

    dispatch({
      type: "SET_WINDOW_MOST_RECENT_EVENT",
      payload: {
        event: nextEvent,
        timestamp: Date.now(),
        isOneshot,
      },
    });

    safeSetTimeout(() => {
      dispatch({ type: "DEQUEUE_WINDOW_EVENT" });
      processingQueueRef.current = false;
    }, displayDurationRef.current);
  }, [state.windowEvents.displayQueue, safeSetTimeout]);

  // Clean up mostRecentEvent after it expires
  useEffect(() => {
    if (state.windowEvents.mostRecentEvent) {
      const timeElapsed = Date.now() - state.windowEvents.mostRecentEvent.timestamp;
      if (timeElapsed < displayDurationRef.current) {
        safeSetTimeout(() => {
          dispatch({ type: "SET_WINDOW_MOST_RECENT_EVENT", payload: null });
        }, displayDurationRef.current - timeElapsed);
      } else {
        dispatch({ type: "SET_WINDOW_MOST_RECENT_EVENT", payload: null });
      }
    }
  }, [state.windowEvents.mostRecentEvent, safeSetTimeout]);

  const latestWindowEvent = useMemo(() => {
    if (
      state.windowEvents.mostRecentEvent &&
      Date.now() - state.windowEvents.mostRecentEvent.timestamp < displayDurationRef.current
    ) {
      return state.windowEvents.mostRecentEvent.event;
    }

    // Show first queue item to prevent flickering
    if (state.windowEvents.displayQueue.length > 0 && !processingQueueRef.current) {
      return state.windowEvents.displayQueue[0];
    }

    return null;
  }, [state.windowEvents.mostRecentEvent, state.windowEvents.displayQueue]);

  // Activity Router - Process incoming events and route by location
  useEffect(() => {
    const routeEvent = (event: ActivityEvent) => {
      const location = getEventLocation(event, activityLocationsRef);

      // Track location for start events so progress/finish events can inherit it
      if ("start" in event) {
        activityLocationsRef.current.set(event.start.activityId, location);
      }

      switch (location) {
        case "window":
          dispatch({ type: "ADD_WINDOW_EVENT", payload: event });

          if ("oneshot" in event) {
            safeSetTimeout(() => {
              dispatch({
                type: "REMOVE_WINDOW_ONESHOT",
                payload: { id: event.oneshot.id },
              });
            }, ONESHOT_CLEANUP_DELAY);
          } else if ("finish" in event) {
            const activityId = event.finish.activityId;

            dispatch({
              type: "FINISH_WINDOW_ACTIVITY",
              payload: { activityId },
            });

            safeSetTimeout(() => {
              dispatch({
                type: "CLEANUP_WINDOW_ACTIVITY_PROGRESS",
                payload: { activityId },
              });
            }, PROGRESS_CLEANUP_DELAY);
          }
          break;

        case "notification":
          // Route to notification system - persistent notification
          if ("oneshot" in event) {
            createActivityNotification(event.oneshot.title, event.oneshot.detail, true);
          } else if ("start" in event) {
            createActivityNotification(event.start.title, event.start.detail, true);
          }
          break;

        case "toast":
          // Route to toast system - auto-dismiss after default duration
          if ("oneshot" in event) {
            createActivityNotification(event.oneshot.title, event.oneshot.detail, false);
          } else if ("start" in event) {
            createActivityNotification(event.start.title, event.start.detail, false);
          }
          break;

        default:
          console.warn("Unknown activity location:", location, event);
          // Fallback to window for unknown locations
          dispatch({ type: "ADD_WINDOW_EVENT", payload: event });
          break;
      }
    };

    // Handle Tauri events from backend and simulated events from UI
    const unlistenProgressStream = listen<ActivityEvent>(ACTIVITY_BROADCASTER_CHANNEL, (event) => {
      routeEvent(event.payload);
    });

    const handleSimulatedEvent = (event: Event) => {
      const customEvent = event as CustomEvent;
      if (customEvent.detail?.payload) {
        const payload = customEvent.detail.payload as ActivityEvent;
        routeEvent(payload);
      }
    };

    window.addEventListener(ACTIVITY_BROADCASTER_CHANNEL, handleSimulatedEvent);

    return () => {
      unlistenProgressStream.then((unlisten) => unlisten());
      window.removeEventListener(ACTIVITY_BROADCASTER_CHANNEL, handleSimulatedEvent);
    };
  }, [safeSetTimeout]);

  const clearEvents = useCallback(() => {
    activityLocationsRef.current.clear();
    dispatch({ type: "CLEAR_ALL" });
  }, []);

  const hasActiveWindowEvents = useMemo(
    () => state.windowEvents.activeActivities.size > 0 || state.windowEvents.oneshotEvents.length > 0,
    [state.windowEvents.activeActivities, state.windowEvents.oneshotEvents]
  );

  const contextValue = useMemo(
    () => ({
      windowEvents: {
        activityEvents: state.windowEvents.activityEvents,
        activeProgressEvents: state.windowEvents.activeProgressEvents,
        oneshotEvents: state.windowEvents.oneshotEvents,
        hasActiveEvents: hasActiveWindowEvents,
        latestEvent: latestWindowEvent,
        displayQueue: state.windowEvents.displayQueue,
        getStartTitleForActivity,
      },
      clearEvents,
    }),
    [
      state.windowEvents.activityEvents,
      state.windowEvents.activeProgressEvents,
      state.windowEvents.oneshotEvents,
      hasActiveWindowEvents,
      latestWindowEvent,
      state.windowEvents.displayQueue,
      getStartTitleForActivity,
      clearEvents,
    ]
  );

  return <ActivityRouterContext.Provider value={contextValue}>{children}</ActivityRouterContext.Provider>;
};
