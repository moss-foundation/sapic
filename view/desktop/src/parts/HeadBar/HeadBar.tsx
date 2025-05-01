import { ActionButton, Divider, IconLabelButton } from "@/components";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";

import { Controls } from "./Controls/Controls";
import { ModeToggle } from "./ModeToggle";
import ActionMenu from "@/components/ActionMenu/ActionMenu";

// Window width threshold for compact mode
const COMPACT_MODE_THRESHOLD = 1000;

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

// Collapsible Menu component that shows action buttons or collapses them into a dropdown
const CollapsibleActionMenu = ({ isCompact, showDebugPanels, setShowDebugPanels, openPanel }) => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  // When not in compact mode, show all buttons
  if (!isCompact) {
    return (
      <div className="flex items-center gap-0">
        <PanelToggleButtons className="mr-1" />
        <ActionButton icon="HeadBarNotifications" iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5" />
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
    );
  }

  // In compact mode, use ActionMenu
  return (
    <ActionMenu
      items={[
        {
          id: "settings",
          type: "action",
          label: "Settings",
          icon: "HeadBarSettings",
        },
        {
          id: "home",
          type: "action",
          label: "Home",
          icon: "TestHeadBarHome",
        },
        {
          id: "logs",
          type: "action",
          label: "Logs",
          icon: "TestHeadBarLogs",
        },
        {
          id: "debug",
          type: "action",
          label: showDebugPanels ? "Hide Debug Panels" : "Show Debug Panels",
          icon: "TestHeadBarDebug",
        },
      ]}
      trigger={
        <ActionButton
          icon="ThreeHorizontalDots"
          iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
          title="More actions"
        />
      }
      open={isMenuOpen}
      onOpenChange={setIsMenuOpen}
      onSelect={(item) => {
        if (item.id === "settings") openPanel("Settings");
        if (item.id === "home") openPanel("Home");
        if (item.id === "logs") openPanel("Logs");
        if (item.id === "debug") setShowDebugPanels(!showDebugPanels);
      }}
    />
  );
};

export const HeadBar = () => {
  const os = type();
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const [isCompact, setIsCompact] = useState(window.innerWidth < COMPACT_MODE_THRESHOLD);

  useEffect(() => {
    // Function to update window dimensions
    const updateWindowDimensions = () => {
      const newWidth = window.innerWidth;
      setIsCompact(newWidth < COMPACT_MODE_THRESHOLD);
    };

    window.addEventListener("resize", updateWindowDimensions);

    updateWindowDimensions();

    return () => window.removeEventListener("resize", updateWindowDimensions);
  }, []);

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
        "header background-(--moss-secondary-background) grid h-full w-screen items-center shadow-[inset_0_-1px_0_0_var(--moss-border-color)]",
        {
          "grid-cols-[max-content_minmax(0px,_1fr)]": os === "macos",
          "grid-cols-[minmax(0px,_1fr)_max-content]": os !== "macos",
        }
      )}
    >
      {os === "macos" && <Controls os={os} />}

      <div
        className={cn("relative mb-0.5 flex w-full items-center overflow-clip", {
          "mr-2 pr-[8px]": os === "macos",
          "ml-2 pr-[8px]": os === "windows" || os === "linux",
        })}
        style={{ overflowClipMargin: 4 }}
        data-tauri-drag-region
      >
        {/* Main content container with proper layout */}
        <div className="flex w-full items-center justify-between" data-tauri-drag-region>
          {/*HeadBar Left-side items*/}
          <div className={isCompact ? "flex items-center gap-0" : "flex items-center gap-3"} data-tauri-drag-region>
            <ActionButton
              icon="HeadBarWindowsMenu"
              iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
              title="Menu"
            />
            <ModeToggle className="mr-2 border-1 border-[var(--moss-headBar-border-color)]" compact={isCompact} />
            <IconLabelButton rightIcon="ChevronDown" title="My Workspace" labelClassName="text-md" />
            <IconLabelButton
              leftIcon="HeadBarVault"
              leftIconClassName="--moss-headBar-icon-primary-text size-4.5"
              title="Vault"
              compact={isCompact}
            />
          </div>

          {/*HeadBar Center items*/}
          <div
            className="flex h-[26px] items-center rounded border border-[var(--moss-headBar-border-color)] bg-[var(--moss-headBar-primary-background)] px-1"
            data-tauri-drag-region
          >
            <IconLabelButton
              leftIcon="HeadBarCollection"
              leftIconClassName="text-(--moss-headBar-icon-primary-text)"
              className={
                isCompact
                  ? "mr-[3px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
                  : "mr-[30px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
              }
              title="Sapic Test Collection"
            />
            <ActionButton
              icon="Reload"
              iconClassName="text-(--moss-headBar-icon-primary-text)"
              customHoverBackground="hover:bg-[var(--moss-headBar-primary-background-hover)]"
              title="Reload"
            />
            <ActionButton
              icon="ThreeVerticalDots"
              iconClassName="text-(--moss-headBar-icon-primary-text)"
              customHoverBackground="hover:bg-[var(--moss-headBar-primary-background-hover)]"
              className="mr-[-4px]"
              title="Reload"
            />
            <Divider />
            <IconLabelButton
              leftIcon="HeadBarGit"
              leftIconClassName="text-(--moss-headBar-icon-primary-text) hover:bg-[var(--moss-headBar-primary-background-hover)]"
              rightIcon="ChevronDown"
              className="hover:bg-[var(--moss-headBar-primary-background-hover)]"
              title="main"
            />
          </div>

          {/*HeadBar Right-side items*/}
          <div className="flex items-center">
            <IconLabelButton
              leftIcon="HeadBarUserAvatar"
              leftIconClassName="text-(--moss-primary) size-4.5"
              rightIcon="ChevronDown"
              title="g10z3r"
              className="mr-2"
              compact={isCompact}
            />

            <CollapsibleActionMenu
              isCompact={isCompact}
              showDebugPanels={showDebugPanels}
              setShowDebugPanels={setShowDebugPanels}
              openPanel={openPanel}
            />
          </div>
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
