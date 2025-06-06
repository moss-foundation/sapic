import { HTMLProps } from "react";

import { cn } from "@/utils";
import { OsType, type } from "@tauri-apps/plugin-os";

import { TauriAppWindowProvider } from "./ControlsContext";
import { LinuxControls } from "./LinuxControls";
import { WindowsControls } from "./WindowsControls";

interface ControlsProps extends HTMLProps<HTMLDivElement> {
  os?: OsType;
  className?: string;
}

export const Controls = ({ os, className, ...props }: ControlsProps) => {
  const osFromTauri = type();

  const switchValue = os || osFromTauri;

  const ControlsComponent = () => {
    switch (switchValue) {
      case "windows":
        return <WindowsControls className={cn(className)} {...props} />;
      case "macos":
        // Return a placeholder element with the appropriate space for native MacOS controls
        return (
          <div className={cn("flex h-full", className)} style={{ width: "72px" }} data-tauri-drag-region {...props} />
        );
      case "linux":
        return <LinuxControls className={cn(className, "py-2.5")} {...props} />;
      default:
        return <WindowsControls className={cn(className)} {...props} />;
    }
  };

  return (
    <TauriAppWindowProvider>
      <ControlsComponent />
    </TauriAppWindowProvider>
  );
};
