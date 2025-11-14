import { useCallback, useEffect, useRef, useState } from "react";

import { useWindowActivityEvents } from "@/hooks/app";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { ActivityEvent } from "@repo/moss-activity-broadcaster";

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

export const StatusBarActivity = () => {
  const { hasActiveEvents, latestEvent, getStartTitleForActivity, displayQueue } = useWindowActivityEvents();
  const [animateIcon, setAnimateIcon] = useState(false);
  const [displayText, setDisplayText] = useState<string | null>(null);
  const [currentEventKey, setCurrentEventKey] = useState<string | null>(null);
  const lastValidTextRef = useRef<string | null>(null);
  const [forceHide, setForceHide] = useState(false);

  // Format the display text for different event types
  const formatEventText = useCallback(
    (event: ActivityEvent | null): string | null => {
      if (!event) return null;

      if ("oneshot" in event) {
        const detail = event.oneshot.detail;
        return detail ? `${event.oneshot.title}: ${detail}` : event.oneshot.title;
      } else if ("start" in event) {
        return `${event.start.title}...`;
      } else if ("progress" in event) {
        const activityId = event.progress.activityId;
        const startTitle = getStartTitleForActivity(activityId);

        if (startTitle && event.progress.detail) {
          return `${startTitle}: ${event.progress.detail}`;
        } else if (startTitle) {
          return `${startTitle}...`;
        } else if (event.progress.detail) {
          return event.progress.detail;
        } else {
          return "Processing...";
        }
      } else if ("finish" in event) {
        // When we get a finish event, we'll handle it but return null for text
        return null;
      }

      return null;
    },
    [getStartTitleForActivity]
  );

  // Get a fallback text when no specific event is being displayed
  const getFallbackText = useCallback((): string | null => {
    // If there's a last valid text, use it
    if (lastValidTextRef.current) {
      return lastValidTextRef.current;
    }

    if (displayQueue.length > 0) {
      return "Processing...";
    }

    if (hasActiveEvents) {
      return "Activity in progress...";
    }

    return null;
  }, [displayQueue.length, hasActiveEvents]);

  useEffect(() => {
    if (!latestEvent) {
      const fallbackText = getFallbackText();
      setDisplayText(fallbackText);

      if (!hasActiveEvents && displayQueue.length === 0) {
        setForceHide(true);
      }
      return;
    }

    setForceHide(false);

    const eventKey = getEventKey(latestEvent);

    // If this is a new event, update the display
    if (eventKey !== currentEventKey) {
      setCurrentEventKey(eventKey);

      if ("finish" in latestEvent) {
        if (!hasActiveEvents && displayQueue.length === 0) {
          setTimeout(() => {
            setForceHide(true);
            lastValidTextRef.current = null;
            setDisplayText(null);
          }, 500);
        }
        return;
      }

      const formattedText = formatEventText(latestEvent);

      if (formattedText) {
        setDisplayText(formattedText);
        lastValidTextRef.current = formattedText;
      } else {
        setDisplayText(getFallbackText());
      }

      setAnimateIcon(true);
    }
  }, [latestEvent, formatEventText, currentEventKey, getFallbackText, hasActiveEvents, displayQueue.length]);

  useEffect(() => {
    if (hasActiveEvents || displayQueue.length > 0) {
      setForceHide(false);
    }
  }, [hasActiveEvents, displayQueue.length]);

  useEffect(() => {
    if (!hasActiveEvents && displayQueue.length === 0 && animateIcon) {
      setAnimateIcon(false);
      lastValidTextRef.current = null;

      setTimeout(() => {
        if (!hasActiveEvents && displayQueue.length === 0) {
          setForceHide(true);
        }
      }, 500);
    } else if ((hasActiveEvents || displayQueue.length > 0) && !animateIcon) {
      setAnimateIcon(true);
    }
  }, [hasActiveEvents, displayQueue, animateIcon]);

  useEffect(() => {
    if (latestEvent) {
      if ("progress" in latestEvent) {
        // console.log(`Progress event: activityId=${latestEvent.progress.activityId}, detail=${latestEvent.progress.detail}`);
      } else if ("finish" in latestEvent) {
        // console.log(
        //   `Finish event: activityId=${latestEvent.finish.activityId}, hasActiveEvents=${hasActiveEvents}, queueLength=${displayQueue.length}`
        // );
      }
    }
  }, [latestEvent, hasActiveEvents, displayQueue.length]);

  if (forceHide || (!hasActiveEvents && displayQueue.length === 0 && !displayText)) {
    return null;
  }

  // Always show some text during active simulation
  const textToDisplay = displayText || getFallbackText();

  return (
    <div className="flex h-full items-center">
      <button className="group flex h-full items-center rounded transition">
        <div className="hover:background-(--moss-statusBarItem-background-hover) flex h-[22px] items-center gap-1.5 rounded px-1">
          <Icon className={cn("size-[14px]", animateIcon && "animate-spin")} icon="Refresh" />
          {textToDisplay && <span className="text-(--moss-statusBarItem-foreground)">{textToDisplay}</span>}
        </div>
      </button>
    </div>
  );
};
