import { useEffect, useState, useCallback, useRef } from "react";
import { Icon } from "@/components";
import { useActivityEvents } from "@/context/ActivityEventsContext";
import { cn } from "@/utils";
import { ActivityEvent } from "@repo/moss-workbench";

export const StatusBarActivity = () => {
  const { hasActiveEvents, latestEvent, getStartTitleForActivity, displayQueue } = useActivityEvents();
  const [animateIcon, setAnimateIcon] = useState(false);
  const [displayText, setDisplayText] = useState<string | null>(null);
  const [currentEventKey, setCurrentEventKey] = useState<string | null>(null);
  // Keep track of the last valid display text to prevent flickering
  const lastValidTextRef = useRef<string | null>(null);
  // Track when we should force-hide the component
  const [forceHide, setForceHide] = useState(false);

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

    // If there are events in the queue, show a generic message
    if (displayQueue.length > 0) {
      return "Processing...";
    }

    // If there are active events but nothing specific to display
    if (hasActiveEvents) {
      return "Activity in progress...";
    }

    return null;
  }, [displayQueue.length, hasActiveEvents]);

  useEffect(() => {
    if (!latestEvent) {
      // Instead of immediately clearing display text, use the fallback
      const fallbackText = getFallbackText();
      setDisplayText(fallbackText);

      // If no latestEvent and no hasActiveEvents and no queue, we should hide
      if (!hasActiveEvents && displayQueue.length === 0) {
        setForceHide(true);
      }
      return;
    }

    // Reset force hide when we have a new event
    setForceHide(false);

    // Get a key for the current event to detect changes
    const eventKey = getEventKey(latestEvent);

    // If this is a new event, update the display
    if (eventKey !== currentEventKey) {
      setCurrentEventKey(eventKey);

      // Handle finish events specially
      if ("finish" in latestEvent) {
        // If this is a finish event, check if we need to hide the component
        if (!hasActiveEvents && displayQueue.length === 0) {
          // Schedule a hide after a short delay
          setTimeout(() => {
            setForceHide(true);
            lastValidTextRef.current = null;
            setDisplayText(null);
          }, 500);
        }
        return;
      }

      // Format the display text based on event type
      const formattedText = formatEventText(latestEvent);

      // Only update if we got valid text
      if (formattedText) {
        setDisplayText(formattedText);
        // Save this as the last valid text
        lastValidTextRef.current = formattedText;
      } else {
        // If no valid text, use fallback
        setDisplayText(getFallbackText());
      }

      // Enable animation for all event types
      setAnimateIcon(true);
    }
  }, [latestEvent, formatEventText, currentEventKey, getFallbackText, hasActiveEvents, displayQueue.length]);

  // Reset forceHide when new activities start
  useEffect(() => {
    if (hasActiveEvents || displayQueue.length > 0) {
      setForceHide(false);
    }
  }, [hasActiveEvents, displayQueue.length]);

  // Update animation state based on active events and display queue
  useEffect(() => {
    if (!hasActiveEvents && displayQueue.length === 0 && animateIcon) {
      setAnimateIcon(false);
      // Also clear the last valid text when everything is done
      lastValidTextRef.current = null;

      // Set forceHide after a short delay when all activities are done
      setTimeout(() => {
        if (!hasActiveEvents && displayQueue.length === 0) {
          setForceHide(true);
        }
      }, 500);
    } else if ((hasActiveEvents || displayQueue.length > 0) && !animateIcon) {
      setAnimateIcon(true);
    }
  }, [hasActiveEvents, displayQueue, animateIcon]);

  // Debug the current event if needed
  useEffect(() => {
    if (latestEvent) {
      if ("progress" in latestEvent) {
        const activityId = latestEvent.progress.activityId;
        const title = getStartTitleForActivity(activityId);
        console.log(`Progress event: activityId=${activityId}, title=${title}, detail=${latestEvent.progress.detail}`);
      } else if ("finish" in latestEvent) {
        console.log(
          `Finish event: activityId=${latestEvent.finish.activityId}, hasActiveEvents=${hasActiveEvents}, queueLength=${displayQueue.length}`
        );
      }
    }
  }, [latestEvent, getStartTitleForActivity, hasActiveEvents, displayQueue.length]);

  // Show status bar item if there are active events, display queue items, or text to display
  // And not force hidden
  if (forceHide || (!hasActiveEvents && displayQueue.length === 0 && !displayText)) {
    return null;
  }

  // Always show some text during active simulation
  const textToDisplay = displayText || getFallbackText();

  return (
    <div className="flex h-full items-center">
      <button className="hover:background-(--moss-statusBar-icon-background-hover) group flex h-full items-center rounded-md px-2 py-1 transition">
        <div className="flex items-center gap-1.5">
          <Icon
            className={cn(
              "size-[14px] flex-shrink-0 text-(--moss-statusBar-icon-secondary-text)",
              animateIcon && "animate-spin"
            )}
            icon="StatusBarProcessing"
          />
          {textToDisplay && <span className="text-(--moss-statusBar-icon-primary-text)">{textToDisplay}</span>}
        </div>
      </button>
    </div>
  );
};
