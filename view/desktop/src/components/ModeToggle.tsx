import { useStreamedCollectionsWithEntries } from "@/hooks";
import { useRequestModeStore } from "@/store/requestMode";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

interface ModeToggleProps {
  className?: string;
}

export const ModeToggle = ({ className }: ModeToggleProps) => {
  const { api } = useTabbedPaneStore();
  const { displayMode, setDisplayMode } = useRequestModeStore();
  const { data: collectionsWithEntries } = useStreamedCollectionsWithEntries();

  const handleSetRequestFirstMode = () => {
    console.log("handleSetRequestFirstMode");
    setDisplayMode("REQUEST_FIRST");
    const allEntries = collectionsWithEntries?.map((collection) => collection.entries).flat();

    allEntries?.forEach((entry) => {
      if (entry.class !== "Request" || entry.path.segments.length === 1) {
        const panel = api?.getPanel(entry.id);
        if (panel) {
          api?.removePanel(panel);
        }
      }
    });
  };

  const handleSetDesignFirstMode = () => {
    console.log("handleSetDesignFirstMode");
    setDisplayMode("DESIGN_FIRST");
  };

  return (
    <div
      className={cn(
        "background-(--moss-display-mode-bg) flex w-full rounded-sm border-1 border-(--moss-display-mode-border) p-px",
        className
      )}
    >
      <div className="grow overflow-hidden">
        <input
          type="radio"
          id="request-first"
          value="REQUEST_FIRST"
          className={cn("peer sr-only")}
          checked={displayMode === "REQUEST_FIRST"}
          onClick={handleSetRequestFirstMode}
        />
        <label
          htmlFor="request-first"
          className="flex min-w-0 cursor-pointer items-center justify-center rounded-sm px-1 py-px leading-5 text-(--moss-display-mode-text) transition-colors duration-300 peer-checked:bg-white peer-checked:text-(--moss-display-mode-text-selected)"
        >
          <span className="w-full truncate text-center">Request mode</span>
        </label>
      </div>

      <div className="grow overflow-hidden">
        <input
          type="radio"
          id="design-first"
          value="DESIGN_FIRST"
          className={cn("peer sr-only")}
          checked={displayMode === "DESIGN_FIRST"}
          onClick={handleSetDesignFirstMode}
        />
        <label
          htmlFor="design-first"
          className="flex min-w-0 cursor-pointer items-center justify-center rounded-sm px-1 py-px leading-5 text-(--moss-display-mode-text) transition-colors duration-300 peer-checked:bg-white peer-checked:text-(--moss-display-mode-text-selected)"
        >
          <span className="w-full truncate text-center">Design mode</span>
        </label>
      </div>
    </div>
  );
};

export default ModeToggle;
