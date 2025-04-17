import React, { createContext, useContext, useState, useEffect, useRef } from "react";
import { ActivityEvent } from "@repo/moss-workbench";
import { listen } from "@tauri-apps/api/event";

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

// Helper function to extract ID from any type of ActivityEvent
const getEventId = (event: ActivityEvent): number => {
  if ("oneshot" in event) return event.oneshot.id;
  if ("start" in event) return event.start.id;
  if ("progress" in event) return event.progress.id;
  if ("finish" in event) return event.finish.id;
  return -1;
};

export const ActivityEventsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [activityEvents, setActivityEvents] = useState<ActivityEvent[]>([]);
  const [activeProgressEvents, setActiveProgressEvents] = useState<Map<string, ActivityEvent[]>>(new Map());
  const [oneshotEvents, setOneshotEvents] = useState<ActivityEvent[]>([]);
  const [activeActivities, setActiveActivities] = useState<Set<string>>(new Set());
  const [startTitles, setStartTitles] = useState<Map<string, string>>(new Map());
  const [mostRecentEvent, setMostRecentEvent] = useState<{
    event: ActivityEvent;
    timestamp: number;
    isOneshot: boolean;
  } | null>(null);

  // Queue for displaying events sequentially in the status bar
  const [displayQueue, setDisplayQueue] = useState<ActivityEvent[]>([]);
  const processingQueueRef = useRef(false);
  const displayDurationRef = useRef(100); // Default 100 ms display time

  const getStartTitleForActivity = (activityId: string): string | null => {
    const title = startTitles.get(activityId);
    if (title) return title;

    const startEvent = activityEvents.find((event) => "start" in event && event.start.activityId === activityId);

    if (startEvent && "start" in startEvent) {
      setStartTitles((prev) => {
        const newMap = new Map(prev);
        newMap.set(activityId, startEvent.start.title);
        return newMap;
      });
      return startEvent.start.title;
    }

    return null;
  };

  // Process the display queue
  useEffect(() => {
    if (displayQueue.length === 0 || processingQueueRef.current) {
      return;
    }

    const processNextEvent = () => {
      processingQueueRef.current = true;

      // Take the next event from the queue
      const nextEvent = displayQueue[0];
      const isOneshot = "oneshot" in nextEvent;

      // Set as most recent for display
      setMostRecentEvent({
        event: nextEvent,
        timestamp: Date.now(),
        isOneshot,
      });

      // Remove from queue after display duration
      setTimeout(() => {
        setDisplayQueue((prev) => {
          // Make sure we still have events before removing
          if (prev.length > 0) {
            return prev.slice(1);
          }
          return prev;
        });
        processingQueueRef.current = false;
      }, displayDurationRef.current);
    };

    processNextEvent();
  }, [displayQueue]);

  // Get the most recent event to display - prioritize oneshot events (highest priority)
  // then show the most recently received progress event
  const latestEvent = React.useMemo(() => {
    if (mostRecentEvent && Date.now() - mostRecentEvent.timestamp < displayDurationRef.current) {
      return mostRecentEvent.event;
    }

    // If there are events in the queue but none being displayed,
    // show the first one to prevent flickering
    if (displayQueue.length > 0 && !processingQueueRef.current) {
      return displayQueue[0];
    }

    return null;
  }, [mostRecentEvent, displayQueue]);

  // Clean up mostRecentEvent after it expires
  useEffect(() => {
    if (mostRecentEvent) {
      const timeElapsed = Date.now() - mostRecentEvent.timestamp;
      if (timeElapsed < displayDurationRef.current) {
        const timer = setTimeout(() => {
          setMostRecentEvent(null);
        }, displayDurationRef.current - timeElapsed);
        return () => clearTimeout(timer);
      } else {
        setMostRecentEvent(null);
      }
    }
  }, [mostRecentEvent]);

  useEffect(() => {
    // Process incoming event
    const processEvent = (event: ActivityEvent) => {
      setActivityEvents((prev) => [...prev, event]);

      // Add event to display queue
      if ("oneshot" in event) {
        // Oneshot events get priority - add them to the front of the queue
        setDisplayQueue((prev) => [event, ...prev]);
      } else if ("finish" in event) {
        // Don't add finish events to display queue as they shouldn't be displayed
        // But we should immediately process the activeActivities cleanup instead of waiting
        const activityId = event.finish.activityId;

        setActiveActivities((prev) => {
          const newSet = new Set(prev);
          newSet.delete(activityId);
          return newSet;
        });

        // Clean up stored start title
        setStartTitles((prev) => {
          const newMap = new Map(prev);
          newMap.delete(activityId);
          return newMap;
        });

        // Schedule cleanup of progress events
        setTimeout(() => {
          setActiveProgressEvents((prev) => {
            const newMap = new Map(prev);
            newMap.delete(activityId);
            return newMap;
          });
        }, 1000);
      } else {
        // Add other events (start, progress) to the end of the queue
        setDisplayQueue((prev) => [...prev, event]);

        // If this is a start event, make sure it's processed quickly
        if ("start" in event) {
          // If queue is empty except for this event, process it immediately
          if (displayQueue.length === 0 && !processingQueueRef.current) {
            setMostRecentEvent({
              event,
              timestamp: Date.now(),
              isOneshot: false,
            });
          }
        }
      }

      // Handle each event type
      if ("oneshot" in event) {
        setOneshotEvents((prev) => [...prev, event]);

        // Auto-remove oneshot event after 1 second
        setTimeout(() => {
          setOneshotEvents((prev) => prev.filter((e) => !("oneshot" in e) || e.oneshot.id !== event.oneshot.id));
        }, 1000);
      } else if ("start" in event) {
        // Create/update the progress events for this activityId
        const activityId = event.start.activityId;

        setStartTitles((prev) => {
          const newMap = new Map(prev);
          newMap.set(activityId, event.start.title);
          return newMap;
        });

        setActiveProgressEvents((prev) => {
          const newMap = new Map(prev);
          if (!newMap.has(activityId)) {
            newMap.set(activityId, []);
          }
          const events = [...(newMap.get(activityId) || []), event];
          newMap.set(
            activityId,
            events.sort((a, b) => getEventId(a) - getEventId(b))
          );
          return newMap;
        });

        setActiveActivities((prev) => {
          const newSet = new Set(prev);
          newSet.add(activityId);
          return newSet;
        });
      } else if ("progress" in event) {
        const activityId = event.progress.activityId;

        setActiveProgressEvents((prev) => {
          const newMap = new Map(prev);
          if (!newMap.has(activityId)) {
            newMap.set(activityId, []);
          }
          const events = [...(newMap.get(activityId) || []), event];
          newMap.set(
            activityId,
            events.sort((a, b) => getEventId(a) - getEventId(b))
          );
          return newMap;
        });

        setActiveActivities((prev) => {
          const newSet = new Set(prev);
          newSet.add(activityId);
          return newSet;
        });
      } else if ("finish" in event) {
        const activityId = event.finish.activityId;

        setActiveProgressEvents((prev) => {
          const newMap = new Map(prev);
          if (!newMap.has(activityId)) {
            newMap.set(activityId, []);
          }
          const events = [...(newMap.get(activityId) || []), event];
          newMap.set(
            activityId,
            events.sort((a, b) => getEventId(a) - getEventId(b))
          );
          return newMap;
        });
      }
    };

    // Handle Tauri events from backend
    const unlistenProgressStream = listen<ActivityEvent>("workbench://activity-indicator", (event) => {
      processEvent(event.payload);
    });

    // Handle simulated events from the UI
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
  }, []);

  const clearEvents = () => {
    setActivityEvents([]);
    setActiveProgressEvents(new Map());
    setOneshotEvents([]);
    setActiveActivities(new Set());
    setStartTitles(new Map());
    setMostRecentEvent(null);
    setDisplayQueue([]);
  };

  return (
    <ActivityEventsContext.Provider
      value={{
        activityEvents,
        activeProgressEvents,
        oneshotEvents,
        hasActiveEvents: activeActivities.size > 0 || oneshotEvents.length > 0,
        latestEvent,
        displayQueue,
        getStartTitleForActivity,
        clearEvents,
      }}
    >
      {children}
    </ActivityEventsContext.Provider>
  );
};
