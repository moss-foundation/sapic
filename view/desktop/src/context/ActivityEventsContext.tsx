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
  // Get start event title for a specific activityId
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
  return -1; // Should never happen
};

export const ActivityEventsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [activityEvents, setActivityEvents] = useState<ActivityEvent[]>([]);
  const [activeProgressEvents, setActiveProgressEvents] = useState<Map<string, ActivityEvent[]>>(new Map());
  const [oneshotEvents, setOneshotEvents] = useState<ActivityEvent[]>([]);
  const [activeActivities, setActiveActivities] = useState<Set<string>>(new Set());
  const [startTitles, setStartTitles] = useState<Map<string, string>>(new Map());
  // Track the most recently received event for immediate display
  const [mostRecentEvent, setMostRecentEvent] = useState<{
    event: ActivityEvent;
    timestamp: number;
    isOneshot: boolean;
  } | null>(null);

  // Queue for displaying events sequentially in the status bar
  const [displayQueue, setDisplayQueue] = useState<ActivityEvent[]>([]);
  const processingQueueRef = useRef(false);
  const displayDurationRef = useRef(100); // Default 1 second display time

  // Get start event title for a specific activityId
  const getStartTitleForActivity = (activityId: string): string | null => {
    // Check the title mapping first
    const title = startTitles.get(activityId);
    if (title) return title;

    // If not found in the mapping, try to find it from the activityEvents
    // This helps when events are processed quickly and the title mapping might not be updated yet
    const startEvent = activityEvents.find((event) => "start" in event && event.start.activityId === activityId);

    if (startEvent && "start" in startEvent) {
      // Update the mapping for future use
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
    // If we have a recent event set by the display queue, use it
    if (mostRecentEvent && Date.now() - mostRecentEvent.timestamp < displayDurationRef.current) {
      return mostRecentEvent.event;
    }

    // If there are events in the queue but none being displayed,
    // show the first one to prevent flickering
    if (displayQueue.length > 0 && !processingQueueRef.current) {
      return displayQueue[0];
    }

    // If no current event is being displayed, return null
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
      // Add to the overall activity events list
      setActivityEvents((prev) => [...prev, event]);

      // Add event to display queue
      if ("oneshot" in event) {
        // Oneshot events get priority - add them to the front of the queue
        setDisplayQueue((prev) => [event, ...prev]);
      } else if ("finish" in event) {
        // Don't add finish events to display queue as they shouldn't be displayed
        // But we should immediately process the activeActivities cleanup instead of waiting
        const activityId = event.finish.activityId;

        // Update activeActivities immediately when a finish event is received
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
        // Add to oneshot events
        setOneshotEvents((prev) => [...prev, event]);

        // Auto-remove oneshot event after 1 second
        setTimeout(() => {
          setOneshotEvents((prev) => prev.filter((e) => !("oneshot" in e) || e.oneshot.id !== event.oneshot.id));
        }, 1000);
      } else if ("start" in event) {
        // Create/update the progress events for this activityId
        const activityId = event.start.activityId;

        // Store the title from the start event for later use with progress events
        setStartTitles((prev) => {
          const newMap = new Map(prev);
          newMap.set(activityId, event.start.title);
          return newMap;
        });

        setActiveProgressEvents((prev) => {
          const newMap = new Map(prev);
          // Initialize with empty array if not exists
          if (!newMap.has(activityId)) {
            newMap.set(activityId, []);
          }
          // Add start event and sort by id
          const events = [...(newMap.get(activityId) || []), event];
          newMap.set(
            activityId,
            events.sort((a, b) => getEventId(a) - getEventId(b))
          );
          return newMap;
        });

        // Mark activity as active
        setActiveActivities((prev) => {
          const newSet = new Set(prev);
          newSet.add(activityId);
          return newSet;
        });
      } else if ("progress" in event) {
        // Update the progress events for this activityId
        const activityId = event.progress.activityId;

        setActiveProgressEvents((prev) => {
          const newMap = new Map(prev);
          // If we receive a progress event but don't have a start event, initialize
          if (!newMap.has(activityId)) {
            newMap.set(activityId, []);
          }
          // Add progress event and sort by id
          const events = [...(newMap.get(activityId) || []), event];
          newMap.set(
            activityId,
            events.sort((a, b) => getEventId(a) - getEventId(b))
          );
          return newMap;
        });

        // Ensure activity is marked as active
        setActiveActivities((prev) => {
          const newSet = new Set(prev);
          newSet.add(activityId);
          return newSet;
        });
      } else if ("finish" in event) {
        // Add finish event to the progress events for this activityId
        const activityId = event.finish.activityId;

        setActiveProgressEvents((prev) => {
          const newMap = new Map(prev);
          if (!newMap.has(activityId)) {
            newMap.set(activityId, []);
          }
          // Add finish event and sort by id
          const events = [...(newMap.get(activityId) || []), event];
          newMap.set(
            activityId,
            events.sort((a, b) => getEventId(a) - getEventId(b))
          );
          return newMap;
        });

        // Note: The active activities cleanup is now handled earlier when we process the event
        // This ensures faster response to finish events
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
