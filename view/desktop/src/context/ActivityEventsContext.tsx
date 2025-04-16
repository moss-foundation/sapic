import React, { createContext, useContext, useState, useEffect } from "react";
import { ActivityEvent } from "@repo/moss-workbench";
import { listen } from "@tauri-apps/api/event";

interface ActivityEventsContextType {
  activityEvents: ActivityEvent[];
  hasActiveEvents: boolean;
  latestEvent: ActivityEvent | null;
  clearEvents: () => void;
}

const ActivityEventsContext = createContext<ActivityEventsContextType>({
  activityEvents: [],
  hasActiveEvents: false,
  latestEvent: null,
  clearEvents: () => {},
});

export const useActivityEvents = () => useContext(ActivityEventsContext);

export const ActivityEventsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [activityEvents, setActivityEvents] = useState<ActivityEvent[]>([]);
  const [activeActivities, setActiveActivities] = useState<Set<string>>(new Set());

  // Get the most recent event to display
  const latestEvent = activityEvents.length > 0 ? activityEvents[activityEvents.length - 1] : null;

  useEffect(() => {
    // Handle Tauri events from backend
    const unlistenProgressStream = listen<ActivityEvent>("workbench://activity-indicator", (event) => {
      setActivityEvents((prev) => [...prev, event.payload]);

      // Track active activities by their IDs
      const payload = event.payload;
      if ("start" in payload) {
        setActiveActivities((prev) => {
          const newSet = new Set(prev);
          newSet.add(payload.start.activityId);
          return newSet;
        });
      } else if ("finish" in payload) {
        setActiveActivities((prev) => {
          const newSet = new Set(prev);
          newSet.delete(payload.finish.activityId);
          return newSet;
        });
      }
    });

    // Handle simulated events from the UI
    const handleSimulatedEvent = (event: Event) => {
      const customEvent = event as CustomEvent;
      if (customEvent.detail?.payload) {
        const payload = customEvent.detail.payload as ActivityEvent;

        setActivityEvents((prev) => [...prev, payload]);

        if ("start" in payload) {
          setActiveActivities((prev) => {
            const newSet = new Set(prev);
            newSet.add(payload.start.activityId);
            return newSet;
          });
        } else if ("finish" in payload) {
          setActiveActivities((prev) => {
            const newSet = new Set(prev);
            newSet.delete(payload.finish.activityId);
            return newSet;
          });
        }
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
    setActiveActivities(new Set());
  };

  return (
    <ActivityEventsContext.Provider
      value={{
        activityEvents,
        hasActiveEvents: activeActivities.size > 0,
        latestEvent,
        clearEvents,
      }}
    >
      {children}
    </ActivityEventsContext.Provider>
  );
};
