import { Icon } from "@/components";
import { DEFAULT_BOTTOM_PANE_HEIGHT } from "@/constants/layout";
import { useChangeAppLayoutState } from "@/hooks/useChangeAppLayoutState";
import { useGetAppLayoutState } from "@/hooks/useGetAppLayoutState";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls/Controls";

export const HeadBar = () => {
  const os = type();
  const { data: appLayoutState } = useGetAppLayoutState();
  const { mutate: changeAppLayoutState } = useChangeAppLayoutState();
  const bottomPaneVisibility = useAppResizableLayoutStore((state) => state.bottomPane.visibility);
  const bottomPaneSetHeight = useAppResizableLayoutStore((state) => state.bottomPane.setHeight);
  const bottomPaneSetVisibility = useAppResizableLayoutStore((state) => state.bottomPane.setVisibility);

  const toggleSidebar = () => {
    if (!appLayoutState) return;

    const currentSidebarSetting = appLayoutState.sidebarSetting || "left";
    const isSidebarVisible = appLayoutState.activeSidebar !== "none";

    changeAppLayoutState({
      activeSidebar: isSidebarVisible ? "none" : currentSidebarSetting,
      sidebarSetting: currentSidebarSetting,
    });
  };

  const toggleBottomPane = () => {
    if (!bottomPaneVisibility) {
      bottomPaneSetVisibility(true);
      bottomPaneSetHeight(DEFAULT_BOTTOM_PANE_HEIGHT);
    } else {
      bottomPaneSetVisibility(false);
    }
  };

  // Determine which sidebar is currently set as the preferred one
  const activeSidebarSetting = appLayoutState?.sidebarSetting || "left";
  const isLeftSidebarMode = activeSidebarSetting === "left";

  const isSidebarVisible = appLayoutState?.activeSidebar !== "none";

  return (
    <header
      data-tauri-drag-region
      className={cn(
        "header background-(--moss-secondary-bg) z-50 grid h-full w-screen items-center shadow-[inset_0_-1px_0_0_var(--moss-border-color)]",
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
        <div className="flex w-full items-center gap-3" data-tauri-drag-region>
          <div className="flex shrink-0 items-center gap-1">
            {/* First button - changes based on sidebar setting */}
            {isLeftSidebarMode ? (
              /* Left Sidebar Toggle */
              <button
                className="hover:background-(--moss-icon-primary-bg-hover) flex size-[30px] items-center justify-center rounded"
                onClick={toggleSidebar}
                title="Toggle Left Sidebar"
              >
                <Icon
                  icon={isSidebarVisible ? "HeadBarLeftSideBarActive" : "HeadBarLeftSideBar"}
                  className="size-[18px] text-(--moss-icon-primary-text)"
                />
              </button>
            ) : (
              /* Bottom Panel Toggle */
              <button
                className="hover:background-(--moss-icon-primary-bg-hover) flex size-[30px] items-center justify-center rounded"
                onClick={toggleBottomPane}
                title="Toggle Bottom Panel"
              >
                <Icon
                  icon={bottomPaneVisibility ? "HeadBarPanelActive" : "HeadBarPanel"}
                  className="size-[18px] text-(--moss-icon-primary-text)"
                />
              </button>
            )}

            {/* Second button - changes based on sidebar setting */}
            {isLeftSidebarMode ? (
              /* Bottom Panel Toggle */
              <button
                className="hover:background-(--moss-icon-primary-bg-hover) flex size-[30px] items-center justify-center rounded"
                onClick={toggleBottomPane}
                title="Toggle Bottom Panel"
              >
                <Icon
                  icon={bottomPaneVisibility ? "HeadBarPanelActive" : "HeadBarPanel"}
                  className="size-[18px] text-(--moss-icon-primary-text)"
                />
              </button>
            ) : (
              /* Right Sidebar Toggle */
              <button
                className="hover:background-(--moss-icon-primary-bg-hover) flex size-[30px] items-center justify-center rounded"
                onClick={toggleSidebar}
                title="Toggle Right Sidebar"
              >
                <Icon
                  icon={isSidebarVisible ? "HeadBarRightSideBarActive" : "HeadBarRightSideBar"}
                  className="size-[18px] text-(--moss-icon-primary-text)"
                />
              </button>
            )}
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
