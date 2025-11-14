import { cn } from "@/utils";
import { useWorkspaceModeStore } from "@/workbench/store/workspaceMode";

import { PlannedBadge } from "./PlannedBadge";

interface WorkspaceModeToggleProps {
  className?: string;
  disabled?: boolean;
}

export const WorkspaceModeToggle = ({ className, disabled = true }: WorkspaceModeToggleProps) => {
  const { displayMode, setDisplayMode } = useWorkspaceModeStore();

  const handleSetLiveMode = () => {
    setDisplayMode("LIVE");
  };

  const handleSetDesignMode = () => {
    setDisplayMode("DESIGN");
  };

  return (
    <div
      className={cn(
        "background-(--moss-workspaceMode-background) @container/mode-toggle border-1 border-(--moss-workspaceMode-border) grid w-full grid-cols-2 items-center rounded-sm p-px",
        className
      )}
    >
      <div className={cn("")}>
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
            "text-(--moss-workspaceMode-foreground) peer-checked:text-(--moss-workspaceMode-foreground-selected) flex min-w-0 cursor-pointer items-center justify-center rounded-sm px-1 py-px leading-5 transition-colors duration-300 peer-checked:bg-white"
          )}
        >
          <span className="w-full truncate text-center">Live</span>
        </label>
      </div>

      <div className={cn("flex items-center justify-center gap-1 overflow-hidden px-2.5")}>
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
            "text-(--moss-workspaceMode-foreground) peer-checked:text-(--moss-workspaceMode-foreground-selected) flex min-w-0 cursor-pointer items-center justify-center rounded-sm px-1 py-px leading-5 transition-colors duration-300 peer-checked:bg-white",
            {
              "cursor-not-allowed opacity-50": disabled,
            }
          )}
        >
          <span className="w-full truncate text-center">Design</span>
        </label>
        <PlannedBadge variant="outlined" className="@min-[260px]/mode-toggle:block hidden" />
      </div>
    </div>
  );
};

export default WorkspaceModeToggle;
