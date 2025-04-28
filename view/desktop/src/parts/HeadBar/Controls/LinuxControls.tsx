import { cva } from "class-variance-authority";
import { useContext, type HTMLProps } from "react";

import { cn } from "@/utils";

import { CommonControls } from "./CommonControls";
import { ControlButton } from "./ControlButton";
import ControlsContext from "./ControlsContext";
import { ControlsIcons } from "./icons";

const linuxControlButtonStyles = cva(`
  size-6 cursor-default rounded-full
  background-(--moss-windowControlsLinux-background) 
  hover:background-(--moss-windowControlsLinux-hoverBackground) 
  active:background-(--moss-windowControlsLinux-activeBackground)  
  text-(--moss-windowControlsLinux-text)
`);

export function LinuxControls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const { isWindowMaximized, minimizeWindow, maximizeWindow, closeWindow } = useContext(ControlsContext);

  return (
    <div className={cn("mr-2.5 flex h-auto items-center gap-3.25", className)} {...props}>
      <CommonControls />
      <ControlButton onClick={minimizeWindow} className={linuxControlButtonStyles()}>
        <ControlsIcons.minimizeWin className="size-[9px]" />
      </ControlButton>
      <ControlButton onClick={maximizeWindow} className={linuxControlButtonStyles()}>
        {isWindowMaximized ? (
          <ControlsIcons.maximizeRestoreWin className="size-[9px]" />
        ) : (
          <ControlsIcons.maximizeWin className="size-2" />
        )}
      </ControlButton>
      <ControlButton onClick={closeWindow} className={linuxControlButtonStyles()}>
        <ControlsIcons.closeWin className="size-2" />
      </ControlButton>
    </div>
  );
}
