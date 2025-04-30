import { ActionButton, Divider, IconLabelButton } from "@/components";
import { useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";

import { Controls } from "./Controls/Controls";
import { ModeToggle } from "./ModeToggle";

// Window width threshold for compact mode
const COMPACT_MODE_THRESHOLD = 860;

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

  // Toggle the dropdown menu
  const toggleMenu = () => {
    setIsMenuOpen(!isMenuOpen);
  };

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

  // In compact mode, show a dropdown menu
  return (
    <div className="relative">
      <ActionButton
        icon="ThreeHorizontalDots"
        iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
        onClick={toggleMenu}
        title="More actions"
      />

      {isMenuOpen && (
        <div className="absolute top-full right-0 z-50 mt-1 w-48 rounded-md border border-[var(--moss-border-color)] bg-[var(--moss-secondary-background)] shadow-lg">
          <div className="py-1">
            <button
              onClick={() => {
                toggleMenu();
                openPanel("Settings");
              }}
              className="text-md flex w-full items-center px-4 py-2 text-left hover:bg-[var(--moss-primary-background-hover)]"
            >
              <span className="mr-2">⚙️</span> Settings
            </button>
            <button
              onClick={() => {
                toggleMenu();
                openPanel("Home");
              }}
              className="text-md flex w-full items-center px-4 py-2 text-left hover:bg-[var(--moss-primary-background-hover)]"
            >
              <span className="mr-2">🏠</span> Home
            </button>
            <button
              onClick={() => {
                toggleMenu();
                openPanel("Logs");
              }}
              className="text-md flex w-full items-center px-4 py-2 text-left hover:bg-[var(--moss-primary-background-hover)]"
            >
              <span className="mr-2">📋</span> Logs
            </button>
            <button
              onClick={() => {
                toggleMenu();
                setShowDebugPanels(!showDebugPanels);
              }}
              className="text-md flex w-full items-center px-4 py-2 text-left hover:bg-[var(--moss-primary-background-hover)]"
            >
              <span className="mr-2">🐞</span> {showDebugPanels ? "Hide Debug Panels" : "Show Debug Panels"}
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export const HeadBar = () => {
  const os = type();
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const [windowWidth, setWindowWidth] = useState(window.innerWidth);
  const [isCompact, setIsCompact] = useState(window.innerWidth < COMPACT_MODE_THRESHOLD);

  useEffect(() => {
    // Function to update window dimensions
    const updateWindowDimensions = () => {
      const newWidth = window.innerWidth;
      setWindowWidth(newWidth);
      setIsCompact(newWidth < COMPACT_MODE_THRESHOLD);
    };

    // Add event listener
    window.addEventListener("resize", updateWindowDimensions);

    // Call handler right away so state gets updated with initial window size
    updateWindowDimensions();

    // Remove event listener on cleanup
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
          <IconLabelButton rightIcon="ChevronDown" title="My Workspace" labelClassName="text-md" compact={isCompact} />
          <IconLabelButton
            leftIcon="HeadBarVault"
            leftIconClassName="--moss-headBar-icon-primary-text size-4.5"
            title="Vault"
            compact={isCompact}
          />
          {/* Add a draggable area that takes up remaining space */}
          <div className="flex-grow" data-tauri-drag-region></div>
        </div>

        {/*HeadBar Center items*/}
        <div
          className="absolute left-1/2 flex h-[26px] -translate-x-1/2 transform items-center rounded border border-[var(--moss-headBar-border-color)] bg-[var(--moss-headBar-primary-background)] px-1"
          data-tauri-drag-region
        >
          <IconLabelButton
            leftIcon="HeadBarCollection"
            leftIconClassName="text-(--moss-headBar-icon-primary-text)"
            className="mr-[30px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
            title="Sapic Test Collection"
            compact={isCompact}
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
            compact={isCompact}
          />
        </div>

        {/*HeadBar Right-side items*/}
        <div className="z-10 ml-auto flex items-center">
          <IconLabelButton
            leftIcon="HeadBarUserAvatar"
            leftIconClassName="text-(--moss-primary) size-4.5"
            rightIcon="ChevronDown"
            title="g10z3r"
            className="mr-2"
            compact={isCompact}
          />
          <ModeToggle className="mr-2 border-1 border-[var(--moss-headBar-border-color)]" compact={isCompact} />

          {/* Replace action buttons with collapsible menu */}
          <CollapsibleActionMenu
            isCompact={isCompact}
            showDebugPanels={showDebugPanels}
            setShowDebugPanels={setShowDebugPanels}
            openPanel={openPanel}
          />
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
