import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls/Controls";
import { useWindowsMenuActions } from "./HeadBarActions";
import { HeadBarLeftItems } from "./HeadBarLeftItems";
import { HeadBarRightItems } from "./HeadBarRightItems";

export const HeadBar = () => {
  const os = type();
  const handleWindowsMenuAction = useWindowsMenuActions();

  return (
    <header
      data-tauri-drag-region
      className={cn(
        "header background-(--moss-primary-background) border-(--moss-border) flex h-full w-screen items-center justify-between border-b"
      )}
    >
      {os === "macos" && <Controls os={os} />}

      <div
        className={cn("relative flex h-full w-full items-center justify-between overflow-clip", {
          "mr-2 pl-2.5 pr-[4px]": os === "macos",
          "ml-[7px]": os === "windows" || os === "linux",
        })}
        style={{ overflowClipMargin: 4 }}
        data-tauri-drag-region
      >
        <HeadBarLeftItems handleWindowsMenuAction={handleWindowsMenuAction} os={os} />

        <HeadBarRightItems os={os} />
      </div>
    </header>
  );
};
