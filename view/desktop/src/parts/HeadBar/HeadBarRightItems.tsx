import { useActiveWorkspace } from "@/hooks";

import CollapsibleActionMenu from "./CollapsibleActionMenu";
import { ModeToggle } from "./ModeToggle";

export interface HeadBarRightItemsProps {
  isMedium: boolean;
  isLarge: boolean;
  breakpoint: string;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  openPanel: (panel: string) => void;
  os: string | null;
  selectedWorkspace?: string | null;
}

export const HeadBarRightItems = ({
  isMedium,
  isLarge,
  showDebugPanels,
  setShowDebugPanels,
  openPanel,
  selectedWorkspace: propSelectedWorkspace,
}: HeadBarRightItemsProps) => {
  const workspace = useActiveWorkspace();
  const selectedWorkspace = propSelectedWorkspace || workspace?.name || null;

  return (
    <div className="flex items-center place-self-end">
      {selectedWorkspace && (
        <ModeToggle className="mr-2 border-1 border-[var(--moss-headBar-border-color)]" compact={isLarge} />
      )}

      <CollapsibleActionMenu
        isCompact={isMedium}
        showDebugPanels={showDebugPanels}
        setShowDebugPanels={setShowDebugPanels}
        openPanel={openPanel}
      />
    </div>
  );
};
