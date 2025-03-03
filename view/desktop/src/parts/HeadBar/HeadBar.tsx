import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls/Controls";

export const HeadBar = () => {
  const os = type();

  // os = "windows";
  // os = "macos";
  // os = "linux";

  return (
    <header
      data-tauri-drag-region
      className={cn("header grid h-full w-screen items-center bg-[#E0E0E0] shadow-[inset_0_-1px_0_0_#C6C6C6]", {
        "grid-cols-[max-content_minmax(0px,_1fr)]": os === "macos",
        "grid-cols-[minmax(0px,_1fr)_max-content]": os !== "macos",
      })}
    >
      {os === "macos" && <Controls os={os} />}

      <div
        className={cn("flex w-full items-center justify-between overflow-clip", {
          "pr-[12px]": os === "macos",
          "px-[16px]": os === "windows" || os === "linux",
        })}
        style={{
          overflowClipMargin: 4,
        }}
        data-tauri-drag-region
      ></div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
