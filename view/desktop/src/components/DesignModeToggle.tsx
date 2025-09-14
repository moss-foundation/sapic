import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";

interface DesignModeToggleProps {
  className?: string;
}

export const DesignModeToggle = ({ className }: DesignModeToggleProps) => {
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
        "background-(--moss-display-mode-bg) @container/mode-toggle flex w-full rounded-sm border-1 border-(--moss-display-mode-border) p-px",
        className
      )}
    >
      <div className="grow overflow-hidden">
        <input
          type="radio"
          id="live"
          value="LIVE"
          className={cn("peer sr-only")}
          checked={displayMode === "LIVE"}
          onChange={handleSetLiveMode}
        />
        <label
          htmlFor="live"
          className="flex min-w-0 cursor-pointer items-center justify-center rounded-sm px-1 py-px leading-5 text-(--moss-display-mode-text) transition-colors duration-300 peer-checked:bg-white peer-checked:text-(--moss-display-mode-text-selected)"
        >
          <span className="w-full truncate text-center">Live</span>
        </label>
      </div>

      <div className="grow overflow-hidden">
        <input
          type="radio"
          id="design"
          value="DESIGN"
          className={cn("peer sr-only")}
          checked={displayMode === "DESIGN"}
          onChange={handleSetDesignMode}
        />
        <label
          htmlFor="design"
          className="flex min-w-0 cursor-pointer items-center justify-center rounded-sm px-1 py-px leading-5 text-(--moss-display-mode-text) transition-colors duration-300 peer-checked:bg-white peer-checked:text-(--moss-display-mode-text-selected)"
        >
          <span className="w-full truncate text-center">Design</span>
        </label>
      </div>
    </div>
  );
};

export default DesignModeToggle;
