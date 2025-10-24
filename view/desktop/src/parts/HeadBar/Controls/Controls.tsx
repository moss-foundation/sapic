import { HTMLProps } from "react";

import { cn } from "@/utils";
import { OsType, type } from "@tauri-apps/plugin-os";

import { TauriAppWindowProvider } from "./ControlsContext";
import { LinuxControls } from "./LinuxControls";
import { WindowsControls } from "./WindowsControls";

interface ControlsProps extends HTMLProps<HTMLDivElement> {
  os?: OsType;
}

export const Controls = ({ os, className, ...props }: ControlsProps) => {
  const osFromTauri = type();

  const switchValue = os || osFromTauri;

  let ControlsComponent: React.ReactNode;

  switch (switchValue) {
    case "windows":
      ControlsComponent = <WindowsControls className={cn(className)} {...props} />;
      break;
    case "linux":
      ControlsComponent = <LinuxControls className={cn(className, "")} {...props} />;
      break;
    case "macos":
      // Return a placeholder element with the appropriate space for native MacOS controls
      ControlsComponent = (
        <div className={cn("flex h-full", className)} style={{ width: "72px" }} data-tauri-drag-region {...props} />
      );
      break;
  }
  return <TauriAppWindowProvider>{ControlsComponent}</TauriAppWindowProvider>;
};

export default Controls;
