import { useEffect } from "react";

import { Icon } from "@/components";
import { useGetWorkspaces } from "@/hooks/useGetWorkspaces";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls/Controls";

export const HeadBar = () => {
  const { data: workspaces } = useGetWorkspaces();

  const os = type();

  const { sideBarPosition, bottomPane, sideBar } = useAppResizableLayoutStore();

  const toggleSidebar = () => {
    sideBar.setVisible(!sideBar.visible);
  };

  const toggleBottomPane = () => {
    bottomPane.setVisible(!bottomPane.visible);
  };

  const theMostRecentWorkspace = workspaces?.reduce((prev, current) => {
    if (!prev.lastOpenedAt || !current.lastOpenedAt) {
      return prev; // or handle undefined case as needed
    }
    return prev.lastOpenedAt > current.lastOpenedAt ? prev : current;
  });

  useEffect(() => {
    console.log("workspaces", workspaces);
  }, [workspaces]);

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
              title={sideBarPosition === "left" ? "Toggle Left Sidebar" : "Toggle Bottom Panel"}
            >
              <Icon
                className="size-[18px] text-(--moss-icon-primary-text)"
                icon={
                  sideBarPosition === "left"
                    ? sideBar.visible
                      ? "HeadBarLeftSideBarActive"
                      : "HeadBarLeftSideBar"
                    : sideBar.visible
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

          <div>{theMostRecentWorkspace?.name}</div>

          {/* Add a draggable area that takes up remaining space */}
          <div className="flex-grow" data-tauri-drag-region></div>
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
