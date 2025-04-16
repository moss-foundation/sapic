import { useEffect, useState } from "react";
import { Icon } from "@/components";
import { useActivityEvents } from "@/context/ActivityEventsContext";
import { cn } from "@/utils";

export const StatusBarActivity = () => {
  const { hasActiveEvents, latestEvent } = useActivityEvents();
  const [animateIcon, setAnimateIcon] = useState(false);
  const [displayText, setDisplayText] = useState<string | null>(null);

  useEffect(() => {
    if (!latestEvent) {
      setDisplayText(null);
      return;
    }

    if ("start" in latestEvent) {
      setDisplayText(latestEvent.start.title);
      setAnimateIcon(true);
    } else if ("progress" in latestEvent) {
      // Extract the progress counter (e.g., "20/27") from the detail
      const progressMatch = latestEvent.progress.detail?.match(/^(\d+\/\d+)/);
      if (progressMatch && progressMatch[1]) {
        setDisplayText(`Indexing: ${progressMatch[1]}`);
        setTimeout(() => {}, 1000);
      } else {
        setDisplayText(latestEvent.progress.detail || null);
      }
      setAnimateIcon(true);
    } else if ("finish" in latestEvent) {
      // Show a completed message briefly
      setDisplayText("Completed");
      setAnimateIcon(false);

      // Clear the message after a delay
      const timer = setTimeout(() => {
        setDisplayText(null);
      }, 1000);

      return () => clearTimeout(timer);
    }
  }, [latestEvent]);

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
