import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";

import { PlannedBadge } from "./PlannedBadge";

interface DesignModeToggleProps {
  className?: string;
  disabled?: boolean;
}

export const DesignModeToggle = ({ className, disabled = true }: DesignModeToggleProps) => {
  const { displayMode, setDisplayMode } = useRequestModeStore();

  const handleSetLiveMode = () => {
    setDisplayMode("LIVE");
  };

  const handleSetDesignMode = () => {
    setDisplayMode("DESIGN");
  };

  return (
    <div
      className={cn(
        "background-(--moss-display-mode-bg) @container/mode-toggle flex w-full items-center rounded-sm border-1 border-(--moss-display-mode-border) p-px",
        className
      )}
    >
      <div className={cn("grow overflow-hidden", { "cursor-not-allowed opacity-50": disabled })}>
        <input
          type="radio"
          id="live"
          value="LIVE"
          className={cn("peer sr-only")}
          checked={displayMode === "LIVE"}
          onChange={handleSetLiveMode}
          disabled={disabled}
        />
        <label
          htmlFor="live"
          className={cn(
            "flex min-w-0 cursor-pointer items-center justify-center rounded-sm px-1 py-px leading-5 text-(--moss-display-mode-text) transition-colors duration-300 peer-checked:bg-white peer-checked:text-(--moss-display-mode-text-selected)",
            {
              "cursor-not-allowed": disabled,
            }
          )}
        >
          <span className="w-full truncate text-center">Live</span>
        </label>
      </div>

      <div className={cn("grow overflow-hidden", { "cursor-not-allowed opacity-50": disabled })}>
        <input
          type="radio"
          id="design"
          value="DESIGN"
          className={cn("peer sr-only")}
          checked={displayMode === "DESIGN"}
          onChange={handleSetDesignMode}
          disabled={disabled}
        />
        <label
          htmlFor="design"
          className={cn(
            "flex min-w-0 cursor-pointer items-center justify-center rounded-sm px-1 py-px leading-5 text-(--moss-display-mode-text) transition-colors duration-300 peer-checked:bg-white peer-checked:text-(--moss-display-mode-text-selected)",
            {
              "cursor-not-allowed": disabled,
            }
          )}
        >
          <span className="w-full truncate text-center">Design</span>
        </label>
      </div>
      <PlannedBadge />
    </div>
  );
};

export default DesignModeToggle;
