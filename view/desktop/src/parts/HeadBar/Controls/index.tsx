import { type HTMLProps } from "react";

import { LinuxControls } from "./LinuxControls";
import { WindowsControls } from "./WindowsControls";

export function Controls({ className, ...props }: HTMLProps<HTMLDivElement>) {
  const platform = window.process?.platform || "win32";

  switch (platform) {
    case "darwin":
      // Use native macOS buttons instead of custom ones
      return null;
    case "linux":
      return <LinuxControls className={className} {...props} />;
    default:
      return <WindowsControls className={className} {...props} />;
  }
}
