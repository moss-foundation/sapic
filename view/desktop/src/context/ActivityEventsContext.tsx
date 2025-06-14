import React, { createContext, useCallback, useContext, useEffect, useMemo, useReducer, useRef } from "react";

import { ActivityEvent } from "@repo/moss-workbench";
import { listen } from "@tauri-apps/api/event";

export const MAX_HISTORY_SIZE = 1000; // Limit number of historical events
export const ONESHOT_CLEANUP_DELAY = 1000; // ms
export const PROGRESS_CLEANUP_DELAY = 1000; // ms
export const DEFAULT_DISPLAY_DURATION = 10; // ms

interface ActivityEventsContextType {
  activityEvents: ActivityEvent[];
  activeProgressEvents: Map<string, ActivityEvent[]>;
  oneshotEvents: ActivityEvent[];
  hasActiveEvents: boolean;
  latestEvent: ActivityEvent | null;
  displayQueue: ActivityEvent[];
  getStartTitleForActivity: (activityId: string) => string | null;
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
  timeoutIds: Set<number>; // Track timeouts for cleanup
}

type ActivityAction =
  | { type: "ADD_EVENT"; payload: ActivityEvent }
  | { type: "REMOVE_ONESHOT"; payload: { id: number } }
  | { type: "FINISH_ACTIVITY"; payload: { activityId: string } }
  | { type: "CLEANUP_ACTIVITY_PROGRESS"; payload: { activityId: string } }
  | { type: "SET_MOST_RECENT_EVENT"; payload: { event: ActivityEvent; timestamp: number; isOneshot: boolean } | null }
  | { type: "DEQUEUE_EVENT" }
  | { type: "CLEAR_ALL" }
  | { type: "ADD_TIMEOUT_ID"; payload: { id: number } }
  | { type: "REMOVE_TIMEOUT_ID"; payload: { id: number } };

function activityReducer(state: ActivityState, action: ActivityAction): ActivityState {
  switch (action.type) {
    case "ADD_EVENT": {
      const event = action.payload;
      const eventType = getEventType(event);
      let newDisplayQueue = [...state.displayQueue];

      // Handle display queue - oneshot events go to the front
      if (eventType === "oneshot") {
        newDisplayQueue = [event, ...newDisplayQueue];
      } else if (eventType !== "finish") {
        newDisplayQueue = [...newDisplayQueue, event];
      }

      // Limit activityEvents history size
      let newActivityEvents = [...state.activityEvents, event];
      if (newActivityEvents.length > MAX_HISTORY_SIZE) {
        newActivityEvents = newActivityEvents.slice(-MAX_HISTORY_SIZE);
      }

      let newOneshotEvents = state.oneshotEvents;
      const newActiveActivities = new Set(state.activeActivities);
      const newStartTitles = new Map(state.startTitles);
      const newActiveProgressEvents = new Map(state.activeProgressEvents);

      if (eventType === "oneshot") {
        newOneshotEvents = [...state.oneshotEvents, event];
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
        activityEvents: newActivityEvents,
        displayQueue: newDisplayQueue,
        oneshotEvents: newOneshotEvents,
        activeActivities: newActiveActivities,
        startTitles: newStartTitles,
        activeProgressEvents: newActiveProgressEvents,
      };
    }

    case "REMOVE_ONESHOT": {
      return {
        ...state,
        oneshotEvents: state.oneshotEvents.filter((e) => !("oneshot" in e) || e.oneshot.id !== action.payload.id),
      };
    }

    case "FINISH_ACTIVITY": {
      const { activityId } = action.payload;
      const newActiveActivities = new Set(state.activeActivities);
      newActiveActivities.delete(activityId);

      const newStartTitles = new Map(state.startTitles);
      newStartTitles.delete(activityId);

      return {
        ...state,
        activeActivities: newActiveActivities,
        startTitles: newStartTitles,
      };
    }

    case "CLEANUP_ACTIVITY_PROGRESS": {
      const { activityId } = action.payload;
      const newActiveProgressEvents = new Map(state.activeProgressEvents);
      newActiveProgressEvents.delete(activityId);

      return {
        ...state,
        activeProgressEvents: newActiveProgressEvents,
      };
    }

    case "SET_MOST_RECENT_EVENT": {
      return {
        ...state,
        mostRecentEvent: action.payload,
      };
    }

    case "DEQUEUE_EVENT": {
      if (state.displayQueue.length === 0) return state;
      return {
        ...state,
        displayQueue: state.displayQueue.slice(1),
      };
    }

    case "CLEAR_ALL": {
      // Clear all timeouts
      state.timeoutIds.forEach((id) => clearTimeout(id));

      return {
        activityEvents: [],
        activeProgressEvents: new Map(),
        oneshotEvents: [],
        activeActivities: new Set(),
        startTitles: new Map(),
        mostRecentEvent: null,
        displayQueue: [],
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

const ActivityEventsContext = createContext<ActivityEventsContextType>({
  activityEvents: [],
  activeProgressEvents: new Map(),
  oneshotEvents: [],
  hasActiveEvents: false,
  latestEvent: null,
  displayQueue: [],
  getStartTitleForActivity: () => null,
  clearEvents: () => {},
});

export const useActivityEvents = () => useContext(ActivityEventsContext);

export const ActivityEventsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const initialState: ActivityState = {
    activityEvents: [],
    activeProgressEvents: new Map(),
    oneshotEvents: [],
    activeActivities: new Set(),
    startTitles: new Map(),
    mostRecentEvent: null,
    displayQueue: [],
    timeoutIds: new Set(),
  };

  const [state, dispatch] = useReducer(activityReducer, initialState);

  const processingQueueRef = useRef(false);
  const displayDurationRef = useRef(DEFAULT_DISPLAY_DURATION);

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
      const title = state.startTitles.get(activityId);
      if (title) return title;

      // If not in map, search in the activity events (less efficient)
      const startEvent = state.activityEvents.find(
        (event) => "start" in event && event.start.activityId === activityId
      );

      if (startEvent && "start" in startEvent) {
        return startEvent.start.title;
      }

      return null;
    },
    [state.startTitles, state.activityEvents]
  );

  // Process events from the display queue
  useEffect(() => {
    if (state.displayQueue.length === 0 || processingQueueRef.current) {
      return;
    }

    processingQueueRef.current = true;
    const nextEvent = state.displayQueue[0];
    const isOneshot = "oneshot" in nextEvent;

    dispatch({
      type: "SET_MOST_RECENT_EVENT",
      payload: {
        event: nextEvent,
        timestamp: Date.now(),
        isOneshot,
      },
    });

    safeSetTimeout(() => {
      dispatch({ type: "DEQUEUE_EVENT" });
      processingQueueRef.current = false;
    }, displayDurationRef.current);
  }, [state.displayQueue, safeSetTimeout]);

  // Clean up mostRecentEvent after it expires
  useEffect(() => {
    if (state.mostRecentEvent) {
      const timeElapsed = Date.now() - state.mostRecentEvent.timestamp;
      if (timeElapsed < displayDurationRef.current) {
        safeSetTimeout(() => {
          dispatch({ type: "SET_MOST_RECENT_EVENT", payload: null });
        }, displayDurationRef.current - timeElapsed);
      } else {
        dispatch({ type: "SET_MOST_RECENT_EVENT", payload: null });
      }
    }
  }, [state.mostRecentEvent, safeSetTimeout]);

  const latestEvent = useMemo(() => {
    if (state.mostRecentEvent && Date.now() - state.mostRecentEvent.timestamp < displayDurationRef.current) {
      return state.mostRecentEvent.event;
    }

    // Show first queue item to prevent flickering
    if (state.displayQueue.length > 0 && !processingQueueRef.current) {
      return state.displayQueue[0];
    }

    return null;
  }, [state.mostRecentEvent, state.displayQueue]);

  // Process incoming events
  useEffect(() => {
    const processEvent = (event: ActivityEvent) => {
      dispatch({ type: "ADD_EVENT", payload: event });

      if ("oneshot" in event) {
        safeSetTimeout(() => {
          dispatch({
            type: "REMOVE_ONESHOT",
            payload: { id: event.oneshot.id },
          });
        }, ONESHOT_CLEANUP_DELAY);
      } else if ("finish" in event) {
        const activityId = event.finish.activityId;

        dispatch({
          type: "FINISH_ACTIVITY",
          payload: { activityId },
        });

        safeSetTimeout(() => {
          dispatch({
            type: "CLEANUP_ACTIVITY_PROGRESS",
            payload: { activityId },
          });
        }, PROGRESS_CLEANUP_DELAY);
      }
    };

    // Handle Tauri events from backend and simulated events from UI
    const unlistenProgressStream = listen<ActivityEvent>("workbench://activity-indicator", (event) => {
      processEvent(event.payload);
    });

    const handleSimulatedEvent = (event: Event) => {
      const customEvent = event as CustomEvent;
      if (customEvent.detail?.payload) {
        const payload = customEvent.detail.payload as ActivityEvent;
        processEvent(payload);
      }
    };

    window.addEventListener("workbench://activity-indicator", handleSimulatedEvent);

    return () => {
      unlistenProgressStream.then((unlisten) => unlisten());
      window.removeEventListener("workbench://activity-indicator", handleSimulatedEvent);
    };
  }, [safeSetTimeout]);

  const clearEvents = useCallback(() => {
    dispatch({ type: "CLEAR_ALL" });
  }, []);

  const hasActiveEvents = useMemo(
    () => state.activeActivities.size > 0 || state.oneshotEvents.length > 0,
    [state.activeActivities, state.oneshotEvents]
  );

  const contextValue = useMemo(
    () => ({
      activityEvents: state.activityEvents,
      activeProgressEvents: state.activeProgressEvents,
      oneshotEvents: state.oneshotEvents,
      hasActiveEvents,
      latestEvent,
      displayQueue: state.displayQueue,
      getStartTitleForActivity,
      clearEvents,
    }),
    [
      state.activityEvents,
      state.activeProgressEvents,
      state.oneshotEvents,
      hasActiveEvents,
      latestEvent,
      state.displayQueue,
      getStartTitleForActivity,
      clearEvents,
    ]
  );

  return <ActivityEventsContext.Provider value={contextValue}>{children}</ActivityEventsContext.Provider>;
};
