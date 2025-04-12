import { Icon } from "@/components";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls/Controls";

export const HeadBar = () => {
  const os = type();

  const primarySideBarPosition = useAppResizableLayoutStore((state) => state.primarySideBarPosition);
  const bottomPane = useAppResizableLayoutStore((state) => state.bottomPane);
  const primarySideBar = useAppResizableLayoutStore((state) => state.primarySideBar);

  const toggleSidebar = () => {
    primarySideBar.setVisible(!primarySideBar.visible);
  };

  const toggleBottomPane = () => {
    bottomPane.setVisible(!bottomPane.visible);
  };

  return (
    <header
      data-tauri-drag-region
      className={cn(
        "header background-(--moss-secondary-background) z-50 grid h-full w-screen items-center shadow-[inset_0_-1px_0_0_var(--moss-border-color)]",
        {
          "grid-cols-[max-content_minmax(0px,_1fr)]": os === "macos",
          "grid-cols-[minmax(0px,_1fr)_max-content]": os !== "macos",
        }
      )}
    >
      {os === "macos" && <Controls os={os} />}

      <div
        className={cn("flex w-full items-center justify-between overflow-clip", {
          "pr-[12px]": os === "macos",
          "px-[16px]": os === "windows" || os === "linux",
        })}
        style={{ overflowClipMargin: 4 }}
        data-tauri-drag-region
      >
        <div className="flex w-full items-center gap-3" data-tauri-drag-region>
          <div className="flex shrink-0 items-center gap-1">
            <button
              className="hover:background-(--moss-icon-primary-background-hover) flex size-[30px] items-center justify-center rounded text-(--moss-icon-primary-text)"
              onClick={toggleSidebar}
              title={primarySideBarPosition === "left" ? "Toggle Left Sidebar" : "Toggle Bottom Panel"}
            >
              <Icon
                className="size-[18px] text-(--moss-icon-primary-text)"
                icon={
                  primarySideBarPosition === "left"
                    ? primarySideBar.visible
                      ? "HeadBarLeftSideBarActive"
                      : "HeadBarLeftSideBar"
                    : primarySideBar.visible
                      ? "HeadBarRightSideBarActive"
                      : "HeadBarRightSideBar"
                }
              />
            </button>

            <button
              className="hover:background-(--moss-icon-primary-background-hover) flex size-[30px] items-center justify-center rounded text-(--moss-icon-primary-text)"
              onClick={toggleBottomPane}
              title="Toggle Bottom Panel"
            >
              <Icon
                className="size-[18px] text-(--moss-icon-primary-text)"
                icon={bottomPane.visible ? "HeadBarPanelActive" : "HeadBarPanel"}
              />
            </button>
          </div>

          {/* Add a draggable area that takes up remaining space */}
          <div className="flex-grow" data-tauri-drag-region></div>
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
