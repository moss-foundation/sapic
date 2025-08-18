import { cn } from "@/utils";
import { OsType } from "@tauri-apps/plugin-os";

import CollapsibleActionMenu from "./CollapsibleActionMenu";
import { Controls } from "./Controls";

export interface HeadBarRightItemsProps {
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  openPanel: (panel: string) => void;
  os: OsType;
}

export const HeadBarRightItems = ({ showDebugPanels, setShowDebugPanels, openPanel, os }: HeadBarRightItemsProps) => {
  const isWindowsOrLinux = os === "windows" || os === "linux";

  return (
    <div
      className={cn("flex h-full items-center justify-end", {
        "gap-2": os === "linux",
      })}
      data-tauri-drag-region
    >
      <div className="flex items-center">
        <CollapsibleActionMenu
          showDebugPanels={showDebugPanels}
          setShowDebugPanels={setShowDebugPanels}
          openPanel={openPanel}
        />
      </div>

      {isWindowsOrLinux && <Controls os={os} />}
    </div>
  );
};
