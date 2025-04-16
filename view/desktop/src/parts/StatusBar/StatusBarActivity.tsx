import { useEffect, useState } from "react";
import { Icon } from "@/components";
import { useActivityEvents } from "@/context/ActivityEventsContext";
import { cn } from "@/utils";
import { ActivityEvent } from "@repo/moss-workbench";

export const StatusBarActivity = () => {
  const { hasActiveEvents, latestEvent, getStartTitleForActivity } = useActivityEvents();
  const [animateIcon, setAnimateIcon] = useState(false);
  const [displayText, setDisplayText] = useState<string | null>(null);
  const [currentEventKey, setCurrentEventKey] = useState<string | null>(null);

  // Generate a unique key for an event to track when it changes
  const getEventKey = (event: ActivityEvent | null): string | null => {
    if (!event) return null;

    if ("oneshot" in event) {
      return `oneshot-${event.oneshot.id}-${event.oneshot.activityId}`;
    } else if ("start" in event) {
      return `start-${event.start.id}-${event.start.activityId}`;
    } else if ("progress" in event) {
      return `progress-${event.progress.id}-${event.progress.activityId}`;
    } else if ("finish" in event) {
      return `finish-${event.finish.id}-${event.finish.activityId}`;
    }

    return null;
  };

  useEffect(() => {
    if (!latestEvent) {
      setDisplayText(null);
      return;
    }

    // Get a key for the current event to detect changes
    const eventKey = getEventKey(latestEvent);

    // If this is a new event, update the display
    if (eventKey !== currentEventKey) {
      setCurrentEventKey(eventKey);

      // Handle different types of events
      if ("oneshot" in latestEvent) {
        // Display oneshot event as "event.title: event.detail"
        const detail = latestEvent.oneshot.detail;
        const displayFormat = detail ? `${latestEvent.oneshot.title}: ${detail}` : latestEvent.oneshot.title;
        setDisplayText(displayFormat);
        setAnimateIcon(true);

        // Auto-hide oneshot events after exactly 1 second
        const timer = setTimeout(() => {
          // Only clear if this is still the current event
          if (getEventKey(latestEvent) === currentEventKey) {
            setDisplayText(null);
            setAnimateIcon(false);
            setCurrentEventKey(null);
          }
        }, 1000);

        return () => clearTimeout(timer);
      } else if ("start" in latestEvent) {
        // Display start of a progress sequence as "event.title..."
        setDisplayText(`${latestEvent.start.title}...`);
        setAnimateIcon(true);
      } else if ("progress" in latestEvent) {
        // Display progress as "event.title: event.detail" where title comes from the start event
        const activityId = latestEvent.progress.activityId;
        const startTitle = getStartTitleForActivity(activityId);

        if (startTitle && latestEvent.progress.detail) {
          setDisplayText(`${startTitle}: ${latestEvent.progress.detail}`);
        } else if (startTitle) {
          setDisplayText(`${startTitle}...`);
        } else if (latestEvent.progress.detail) {
          // Fallback if no start title is found
          setDisplayText(latestEvent.progress.detail);
        } else {
          setDisplayText("Processing...");
        }

        setAnimateIcon(true);
      }
      // We don't display finish events as per the requirements
    }
  }, [latestEvent, getStartTitleForActivity, currentEventKey]);

  // Also update animation state based on active events
  useEffect(() => {
    if (!hasActiveEvents && animateIcon) {
      setAnimateIcon(false);
    }
  }, [hasActiveEvents, animateIcon]);

  if (!hasActiveEvents && !displayText) {
    return null;
  }

  return (
    <div className="flex h-full items-center">
      <button className="hover:background-(--moss-icon-primary-background-hover) group flex h-full items-center rounded-md px-2 py-1 transition">
        <div className="flex items-center gap-1.5">
          <Icon
            className={cn("size-[14px] flex-shrink-0 text-(--moss-icon-primary-text)", animateIcon && "animate-spin")}
            icon="StatusBarProcessing"
          />
          {displayText && <span className="text-black">{displayText}</span>}
        </div>
      </button>
    </div>
  );
};
