import { useCallback, useMemo } from "react";

import { useWindowActivityEvents } from "@/hooks";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { LocalizedString } from "@repo/base";
import { ActivityEvent } from "@repo/ipc";

export const StatusBarActivity = () => {
  const { hasActiveEvents, latestEvent, getStartTitleForActivity, displayQueue } = useWindowActivityEvents();

  // Helper to convert LocalizedString to string
  const localizeString = useCallback((localized: LocalizedString | undefined): string | undefined => {
    return localized?.fallback;
  }, []);

  // Format the display text for different event types
  const formatEventText = useCallback(
    (event: ActivityEvent | null): string | null => {
      if (!event) return null;

      if ("oneshot" in event) {
        const title = localizeString(event.oneshot.title);
        const detail = localizeString(event.oneshot.detail);
        return detail ? `${title}: ${detail}` : title || null;
      } else if ("start" in event) {
        const title = localizeString(event.start.title);
        return title ? `${title}...` : null;
      } else if ("progress" in event) {
        const activityId = event.progress.activityId;
        const startTitleLocalized = getStartTitleForActivity(activityId);
        const startTitle = startTitleLocalized ? localizeString(startTitleLocalized) : undefined;
        const detail = localizeString(event.progress.detail);

        if (startTitle && detail) {
          return `${startTitle}: ${detail}`;
        } else if (startTitle) {
          return `${startTitle}...`;
        } else if (detail) {
          return detail;
        } else {
          return "Processing...";
        }
      } else if ("finish" in event) {
        return null;
      }

      return null;
    },
    [getStartTitleForActivity, localizeString]
  );

  // Get a fallback text when no specific event is being displayed
  const getFallbackText = useMemo((): string | null => {
    if (displayQueue.length > 0) {
      return "Processing...";
    }

    if (hasActiveEvents) {
      return "Activity in progress...";
    }

    return null;
  }, [displayQueue.length, hasActiveEvents]);

  // Compute display text directly from latest event (derived state)
  const displayText = useMemo(() => {
    if (!latestEvent) {
      return getFallbackText;
    }

    if ("finish" in latestEvent) {
      return null;
    }

    return formatEventText(latestEvent) || getFallbackText;
  }, [latestEvent, formatEventText, getFallbackText]);

  // Determine if we should show the icon animation
  const animateIcon = hasActiveEvents || displayQueue.length > 0;

  // Determine if we should hide completely
  const shouldHide = !hasActiveEvents && displayQueue.length === 0 && !displayText;

  if (shouldHide) {
    return null;
  }

  // Text to display
  const textToDisplay = displayText || getFallbackText;

  return (
    <div className="flex h-full items-center">
      <button className="group flex h-full items-center rounded transition">
        <div className="hover:background-(--moss-secondary-background-hover) flex h-[22px] items-center gap-1.5 rounded px-1">
          <Icon className={cn("size-[14px]", animateIcon && "animate-spin")} icon="Refresh" />
          {textToDisplay && <span className="text-(--moss-secondary-foreground)">{textToDisplay}</span>}
        </div>
      </button>
    </div>
  );
};
