import { ActionButton, IconLabelButton } from "@/components";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls/Controls";
import { ModeToggle } from "./ModeToggle";

interface PanelToggleButtonsProps {
  className?: string;
}

const PanelToggleButtons = ({ className }: PanelToggleButtonsProps) => {
  const { sideBarPosition, bottomPane, sideBar } = useAppResizableLayoutStore();

  const toggleSidebar = () => {
    sideBar.setVisible(!sideBar.visible);
  };

  const toggleBottomPane = () => {
    bottomPane.setVisible(!bottomPane.visible);
  };

  return (
    <div className={cn("flex shrink-0 -space-x-0.5", className)}>
      {sideBarPosition === "left" ? (
        <>
          <ActionButton
            iconClassName="size-4.5 text-(--moss-icon-primary-text)"
            icon={sideBar.visible ? "HeadBarLeftSideBarActive" : "HeadBarLeftSideBar"}
            onClick={toggleSidebar}
            title="Toggle Left Sidebar"
          />
          <ActionButton
            iconClassName="size-4.5 text-(--moss-icon-primary-text)"
            icon={bottomPane.visible ? "HeadBarPanelActive" : "HeadBarPanel"}
            onClick={toggleBottomPane}
            title="Toggle Bottom Panel"
          />
        </>
      ) : (
        <>
          <ActionButton
            iconClassName="size-4.5 text-(--moss-icon-primary-text)"
            icon={bottomPane.visible ? "HeadBarPanelActive" : "HeadBarPanel"}
            onClick={toggleBottomPane}
            title="Toggle Bottom Panel"
          />
          <ActionButton
            iconClassName="size-4.5 text-(--moss-icon-primary-text)"
            icon={sideBar.visible ? "HeadBarRightSideBarActive" : "HeadBarRightSideBar"}
            onClick={toggleSidebar}
            title="Toggle Right Sidebar"
          />
        </>
      )}
    </div>
  );
};

export const HeadBar = () => {
  const os = type();

  const onOpenSettings = () => {
    const api = useTabbedPaneStore.getState().api;
    if (api?.getPanel("Settings") !== undefined) {
      api.getPanel("Settings")?.focus();
      return;
    }

    api?.addPanel({
      id: "Settings",
      component: "Settings",
      title: "Settings",
      renderer: "onlyWhenVisible",
    });
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
          {/* Add a draggable area that takes up remaining space */}
          <div className="flex-grow" data-tauri-drag-region></div>
        </div>
        <IconLabelButton
          leftIcon="HeadBarUserAvatar"
          leftIconClassName="text-(--moss-primary) size-4.5"
          rightIcon="ChevronDown"
          title="g10z3r"
          className="mr-2"
        />
        <ModeToggle />
        <div className="flex items-center gap-0">
          <PanelToggleButtons className="mr-1" />
          <ActionButton icon="HeadBarNotifications" iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5" />
          <ActionButton
            icon="HeadBarSettings"
            iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
            onClick={onOpenSettings}
            title="Settings"
          />
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
