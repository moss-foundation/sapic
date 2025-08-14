import { useActiveWorkspace } from "@/hooks";
import { cn } from "@/utils";
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
}

export const HeadBarRightItems = ({
  isMedium,
  isLarge,
  showDebugPanels,
  setShowDebugPanels,
  openPanel,
  os,
}: HeadBarRightItemsProps) => {
  const { hasActiveWorkspace } = useActiveWorkspace();

  const isWindowsOrLinux = os === "windows" || os === "linux";

  return (
    <div
      className={cn("flex h-full items-center justify-end", {
        "gap-2": os === "linux",
      })}
      data-tauri-drag-region
    >
      <div className="flex items-center">
        {hasActiveWorkspace && (
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
