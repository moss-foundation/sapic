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
        "background-(--moss-display-mode-bg) flex w-full overflow-hidden rounded-sm border-1 border-(--moss-display-mode-border) p-px",
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
          className="flex grow cursor-pointer items-center justify-center truncate rounded-sm p-px leading-5 text-(--moss-display-mode-text) transition-colors duration-300 peer-checked:bg-white peer-checked:text-(--moss-display-mode-text-selected)"
        >
          Request mode
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
          className="flex grow cursor-pointer items-center justify-center truncate rounded-sm p-px leading-5 text-(--moss-display-mode-text) transition-colors duration-300 peer-checked:bg-white peer-checked:text-(--moss-display-mode-text-selected)"
        >
          Design mode
        </label>
      </div>
    </div>
  );
};

export default ModeToggle;
