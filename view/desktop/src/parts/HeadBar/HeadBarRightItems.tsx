import { useActiveWorkspace } from "@/hooks";
import { OsType } from "@tauri-apps/plugin-os";

import CollapsibleActionMenu from "./CollapsibleActionMenu";
import { Controls } from "./Controls";
import { ModeToggle } from "./ModeToggle";

export interface HeadBarRightItemsProps {
  isMedium: boolean;
  isLarge: boolean;
  breakpoint: string;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  openPanel: (panel: string) => void;
  os: OsType;
  selectedWorkspace?: string | null;
}

export const HeadBarRightItems = ({
  isMedium,
  isLarge,
  showDebugPanels,
  setShowDebugPanels,
  openPanel,
  os,
  selectedWorkspace: propSelectedWorkspace,
}: HeadBarRightItemsProps) => {
  const workspace = useActiveWorkspace();
  const selectedWorkspace = propSelectedWorkspace || workspace?.name || null;

  const isWindowsOrLinux = os === "windows" || os === "linux";

  return (
    <div className="flex h-full items-center justify-end" data-tauri-drag-region>
      <div className="flex items-center">
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

      {isWindowsOrLinux && <Controls os={os} />}
    </div>
  );
};
