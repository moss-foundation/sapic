import React, { createContext, useContext, useState, useEffect } from "react";
import { ActivityEvent } from "@repo/moss-workbench";
import { listen } from "@tauri-apps/api/event";

interface ActivityEventsContextType {
  activityEvents: ActivityEvent[];
  activeProgressEvents: Map<string, ActivityEvent[]>;
  oneshotEvents: ActivityEvent[];
  hasActiveEvents: boolean;
  latestEvent: ActivityEvent | null;
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

  // Get start event title for a specific activityId
  const getStartTitleForActivity = (activityId: string): string | null => {
    return startTitles.get(activityId) || null;
  };

  // Get the most recent event to display - prioritize oneshot events (highest priority)
  // then show the most recently received progress event
  const latestEvent = React.useMemo(() => {
    // If we have a recent oneshot event, it takes priority
    if (mostRecentEvent && mostRecentEvent.isOneshot && Date.now() - mostRecentEvent.timestamp < 1000) {
      return mostRecentEvent.event;
    }

    // If there are oneshot events, they take priority over progress events
    if (oneshotEvents.length > 0) {
      return oneshotEvents[oneshotEvents.length - 1];
    }

    // If we have a recent non-oneshot event, show it
    if (mostRecentEvent && Date.now() - mostRecentEvent.timestamp < 1000 && !("finish" in mostRecentEvent.event)) {
      return mostRecentEvent.event;
    }

    // Otherwise, find the latest non-finish event from active progress events
    if (activeActivities.size > 0) {
      // Find the latest progress event by finding the max id across all active events
      let latestProgressEvent: ActivityEvent | null = null;
      let maxId = -1;

      for (const activityId of activeActivities) {
        const events = activeProgressEvents.get(activityId) || [];

        // Filter out finish events since they shouldn't be displayed
        const nonFinishEvents = events.filter((event) => !("finish" in event));

        if (nonFinishEvents.length === 0) continue;

        const latest = nonFinishEvents[nonFinishEvents.length - 1];

        if (latest) {
          const id = getEventId(latest);
          if (id > maxId) {
            maxId = id;
            latestProgressEvent = latest;
          }
        }
      }

      if (latestProgressEvent) return latestProgressEvent;
    }

    // If no events at all, return null
    return null;
  }, [activeActivities, activeProgressEvents, oneshotEvents, mostRecentEvent]);

  // Clean up mostRecentEvent after it expires
  useEffect(() => {
    if (mostRecentEvent) {
      const timeElapsed = Date.now() - mostRecentEvent.timestamp;
      if (timeElapsed < 1000) {
        const timer = setTimeout(() => {
          setMostRecentEvent(null);
        }, 1000 - timeElapsed);
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

      // Set this as the most recent event for immediate display
      const isOneshot = "oneshot" in event;
      setMostRecentEvent({
        event,
        timestamp: Date.now(),
        isOneshot,
      });

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

        // Remove activity from active set after a delay
        setTimeout(() => {
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

          // Keep finished events for a while, then clean them up
          setTimeout(() => {
            setActiveProgressEvents((prev) => {
              const newMap = new Map(prev);
              newMap.delete(activityId);
              return newMap;
            });
          }, 2000);
        }, 1000);
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
  };

  return (
    <ActivityEventsContext.Provider
      value={{
        activityEvents,
        activeProgressEvents,
        oneshotEvents,
        hasActiveEvents: activeActivities.size > 0 || oneshotEvents.length > 0,
        latestEvent,
        getStartTitleForActivity,
        clearEvents,
      }}
    >
      {children}
    </ActivityEventsContext.Provider>
  );
};
