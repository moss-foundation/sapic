import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";
import { Icon } from "@/components";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useChangeAppLayoutState } from "@/hooks/useChangeAppLayoutState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";

import { Controls } from "./Controls/Controls";

export const HeadBar = () => {
  const os = type();
  const { data: appLayoutState } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();
  const { bottomPane } = useAppResizableLayoutStore((state) => state);

  const toggleSidebar = () => {
    if (!appLayoutState) return;

    const sidebarType = appLayoutState.sidebarSetting || "left";

    changeAppLayoutState({
      activeSidebar: appLayoutState.activeSidebar === "none" ? sidebarType : "none",
      sidebarSetting: sidebarType,
    });
  };

  // Determine which sidebar is currently set as the preferred one
  const activeSidebarSetting = appLayoutState?.sidebarSetting || "left";
  const isLeftSidebarMode = activeSidebarSetting === "left";

  const isSidebarVisible = appLayoutState?.activeSidebar !== "none";

  return (
    <header
      data-tauri-drag-region
      className={cn(
        "header grid h-full w-screen items-center bg-[var(--moss-headBar-background)] shadow-[var(--moss-headBar-shadow)]",
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
        style={{
          overflowClipMargin: 4,
        }}
        data-tauri-drag-region
      >
        <div className="flex w-full items-center gap-3">
          <div className="flex shrink-0 items-center gap-1">
            {/* First button - changes based on sidebar setting */}
            {isLeftSidebarMode ? (
              /* Left Sidebar Toggle */
              <button
                className="flex size-[30px] items-center justify-center rounded hover:bg-[var(--moss-headBar-hover-background)]"
                onClick={toggleSidebar}
                title="Toggle Left Sidebar"
              >
                <Icon
                  icon={isSidebarVisible ? "HeadBarLeftSideBarActive" : "HeadBarLeftSideBar"}
                  className="size-[18px] text-[var(--moss-headBar-icon-color)]"
                />
              </button>
            ) : (
              /* Bottom Panel Toggle */
              <button
                className="flex size-[30px] items-center justify-center rounded hover:bg-[var(--moss-headBar-hover-background)]"
                onClick={() => bottomPane.setVisibility(!bottomPane.visibility)}
                title="Toggle Bottom Panel"
              >
                <Icon
                  icon={bottomPane.visibility ? "HeadBarPanelActive" : "HeadBarPanel"}
                  className="size-[18px] text-[var(--moss-headBar-icon-color)]"
                />
              </button>
            )}

            {/* Second button - changes based on sidebar setting */}
            {isLeftSidebarMode ? (
              /* Bottom Panel Toggle */
              <button
                className="flex size-[30px] items-center justify-center rounded hover:bg-[var(--moss-headBar-hover-background)]"
                onClick={() => bottomPane.setVisibility(!bottomPane.visibility)}
                title="Toggle Bottom Panel"
              >
                <Icon
                  icon={bottomPane.visibility ? "HeadBarPanelActive" : "HeadBarPanel"}
                  className="size-[18px] text-[var(--moss-headBar-icon-color)]"
                />
              </button>
            ) : (
              /* Right Sidebar Toggle */
              <button
                className="flex size-[30px] items-center justify-center rounded hover:bg-[var(--moss-headBar-hover-background)]"
                onClick={toggleSidebar}
                title="Toggle Right Sidebar"
              >
                <Icon
                  icon={isSidebarVisible ? "HeadBarRightSideBarActive" : "HeadBarRightSideBar"}
                  className="size-[18px] text-[var(--moss-headBar-icon-color)]"
                />
              </button>
            )}
          </div>
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
