import { cn } from "@/utils";
import { OsType } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls";
import PanelToggleButtons from "./PanelToggleButtons";

export interface HeadBarRightItemsProps {
  os: OsType;
}

export const HeadBarRightItems = ({ os }: HeadBarRightItemsProps) => {
  const isWindowsOrLinux = os === "windows" || os === "linux";

  return (
    <div className={cn("flex h-full items-center justify-end gap-5")} data-tauri-drag-region>
      <PanelToggleButtons />

      {isWindowsOrLinux && <Controls os={os} />}
    </div>
  );
};
