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

  const toggleSidebar = (position: "left" | "right") => {
    if (!appLayoutState) return;

    if (appLayoutState.activeSidebar === position) {
      changeAppLayoutState({
        activeSidebar: "none",
      });
    } else {
      changeAppLayoutState({
        activeSidebar: position,
      });
    }
  };

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
        <div className="z-50 flex items-center gap-3">
          <div className="flex items-center">
            {/* Left sidebar toggle button */}
            <button
              className="flex size-[30px] items-center justify-center rounded hover:bg-[var(--moss-headBar-hover-background)]"
              onClick={() => toggleSidebar("left")}
            >
              <Icon
                icon={appLayoutState?.activeSidebar === "left" ? "HeadBarLeftSideBarActive" : "HeadBarLeftSideBar"}
                className="size-[18px] text-[var(--moss-headBar-icon-color)]"
              />
            </button>

            {/* Bottom panel toggle button */}
            <button
              className="flex size-[30px] items-center justify-center rounded hover:bg-[var(--moss-headBar-hover-background)]"
              onClick={() => bottomPane.setVisibility(!bottomPane.visibility)}
            >
              <Icon
                icon={bottomPane.visibility ? "HeadBarPanelActive" : "HeadBarPanel"}
                className="size-[18px] text-[var(--moss-headBar-icon-color)]"
              />
            </button>

            {/* Right sidebar toggle button */}
            <button
              className="flex size-[30px] items-center justify-center rounded hover:bg-[var(--moss-headBar-hover-background)]"
              onClick={() => toggleSidebar("right")}
            >
              <Icon
                icon={appLayoutState?.activeSidebar === "right" ? "HeadBarRightSideBarActive" : "HeadBarRightSideBar"}
                className="size-[18px] text-[var(--moss-headBar-icon-color)]"
              />
            </button>
          </div>
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
