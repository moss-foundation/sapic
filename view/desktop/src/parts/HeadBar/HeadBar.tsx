import { ActionButton, Divider, IconLabelButton } from "@/components";
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
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();

  const openPanel = (panelType: string) => {
    try {
      // Use setTimeout to prevent race condition during initialization
      setTimeout(() => {
        const api = useTabbedPaneStore.getState().api;
        if (!api) return;

        try {
          if (api.getPanel(panelType) !== undefined) {
            api.getPanel(panelType)?.focus();
            return;
          }

          api.addPanel({
            id: panelType,
            component: panelType,
            title: panelType,
            renderer: "onlyWhenVisible",
          });
        } catch (error) {
          console.error(`Error opening ${panelType} panel:`, error);
        }
      }, 0);
    } catch (error) {
      console.error(`Error in open${panelType}:`, error);
    }
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
        className={cn("relative mb-0.5 flex w-full items-center overflow-clip", {
          "pr-[12px]": os === "macos",
          "px-[16px]": os === "windows" || os === "linux",
        })}
        style={{ overflowClipMargin: 4 }}
        data-tauri-drag-region
      >
        {/*HeadBar Left-side items*/}
        <div className="z-10 flex items-center gap-3" data-tauri-drag-region>
          <ActionButton
            icon="HeadBarWindowsMenu"
            iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
            title="Menu"
          />
          <IconLabelButton rightIcon="ChevronDown" title="My Workspace" labelClassName="text-md" />
          <IconLabelButton
            leftIcon="HeadBarVault"
            leftIconClassName="--moss-headBar-icon-primary-text size-4.5"
            title="Vault"
          />
          {/* Add a draggable area that takes up remaining space */}
          <div className="flex-grow" data-tauri-drag-region></div>
        </div>

        {/*HeadBar Center items*/}
        <div
          className="absolute left-1/2 flex h-[26px] -translate-x-1/2 transform items-center rounded border border-[var(--moss-headBar-border-color)] bg-[var(--moss-headBar-primary-background)]"
          data-tauri-drag-region
        >
          <IconLabelButton
            leftIcon="HeadBarCollection"
            leftIconClassName="text-(--moss-headBar-icon-primary-text)"
            className="hover:bg-[var(--moss-headBar-primary-background-hover)]"
            title="Sapic Test Collection"
          />
          <div className="flex items-center gap-0.5">
            <ActionButton
              icon="Reload"
              iconClassName="text-(--moss-headBar-icon-primary-text)"
              className="hover:bg-[var(--moss-headBar-primary-background-hover)]"
              title="Reload"
            />
            <ActionButton
              icon="ThreeVerticalDots"
              iconClassName="text-(--moss-headBar-icon-primary-text)"
              className="hover:bg-[var(--moss-headBar-primary-background-hover)]"
              title="Reload"
            />
            <Divider />
            <IconLabelButton
              leftIcon="HeadBarGit"
              leftIconClassName="text-(--moss-headBar-icon-primary-text)"
              rightIcon="ChevronDown"
              className="hover:bg-[var(--moss-headBar-primary-background-hover)]"
              title="main"
            />
          </div>
        </div>

        {/*HeadBar Right-side items*/}
        <div className="z-10 ml-auto flex items-center">
          <IconLabelButton
            leftIcon="HeadBarUserAvatar"
            leftIconClassName="text-(--moss-primary) size-4.5"
            rightIcon="ChevronDown"
            title="g10z3r"
            className="mr-2"
          />
          <ModeToggle className="mr-2" />
          <div className="flex items-center gap-0">
            <PanelToggleButtons className="mr-1" />
            <ActionButton
              icon="HeadBarNotifications"
              iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
            />
            <ActionButton
              icon="HeadBarSettings"
              iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
              onClick={() => openPanel("Settings")}
              title="Settings"
            />
            <ActionButton
              icon="TestHeadBarHome"
              iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
              onClick={() => openPanel("Home")}
              title="Home"
            />
            <ActionButton
              icon="TestHeadBarLogs"
              iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
              onClick={() => openPanel("Logs")}
              title="Logs"
            />
            <ActionButton
              icon="TestHeadBarDebug"
              iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
              onClick={() => setShowDebugPanels(!showDebugPanels)}
              title={showDebugPanels ? "Hide Debug Panels" : "Show Debug Panels"}
            />
          </div>
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
