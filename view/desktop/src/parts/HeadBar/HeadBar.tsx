import { ActionButton, Divider, IconLabelButton } from "@/components";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";

import { Controls } from "./Controls/Controls";
import { ModeToggle } from "./ModeToggle";
import ActionMenu from "@/components/ActionMenu/ActionMenu";
import CollapsibleActionMenu from "./CollapsibleActionMenu";
import { userMenuItems, gitBranchMenuItems, windowsMenuItems } from "./mockHeadBarData";
import { collectionActionMenuItems, workspaceMenuItems } from "./HeadBarData";

// Window width threshold for compact mode
const COMPACT_MODE_THRESHOLD = 1000;

interface HeadBarLeftItemsProps {
  isCompact: boolean;
  windowsMenuOpen: boolean;
  setWindowsMenuOpen: (open: boolean) => void;
  handleWindowsMenuAction: (action: string) => void;
  workspaceMenuOpen: boolean;
  setWorkspaceMenuOpen: (open: boolean) => void;
  handleWorkspaceMenuAction: (action: string) => void;
  os: string | null;
}

const HeadBarLeftItems = ({
  isCompact,
  windowsMenuOpen,
  setWindowsMenuOpen,
  handleWindowsMenuAction,
  workspaceMenuOpen,
  setWorkspaceMenuOpen,
  handleWorkspaceMenuAction,
  os,
}: HeadBarLeftItemsProps) => {
  return (
    <div className={isCompact ? "flex items-center gap-0" : "flex items-center gap-3"} data-tauri-drag-region>
      <ActionMenu
        items={windowsMenuItems}
        trigger={
          <ActionButton
            icon="HeadBarWindowsMenu"
            iconClassName="text-(--moss-headBar-icon-primary-text) size-4.5"
            title="Menu"
          />
        }
        open={windowsMenuOpen}
        onOpenChange={setWindowsMenuOpen}
        onSelect={(item) => {
          handleWindowsMenuAction(item.id);
        }}
      />
      {(os === "windows" || os === "linux") && (
        <ModeToggle className="mr-2 border-1 border-[var(--moss-headBar-border-color)]" compact={isCompact} />
      )}
      <ActionMenu
        items={workspaceMenuItems}
        trigger={<IconLabelButton rightIcon="ChevronDown" title="My Workspace" labelClassName="text-md" />}
        open={workspaceMenuOpen}
        onOpenChange={setWorkspaceMenuOpen}
        onSelect={(item) => {
          handleWorkspaceMenuAction(item.id);
        }}
      />
      <IconLabelButton
        leftIcon="HeadBarVault"
        leftIconClassName="--moss-headBar-icon-primary-text size-4.5"
        title="Vault"
        compact={isCompact}
      />
    </div>
  );
};

interface HeadBarCenterItemsProps {
  isCompact: boolean;
  gitMenuOpen: boolean;
  setGitMenuOpen: (open: boolean) => void;
  handleGitMenuAction: (action: string) => void;
  collectionActionMenuOpen: boolean;
  setCollectionActionMenuOpen: (open: boolean) => void;
  handleCollectionActionMenuAction: (action: string) => void;
}

const HeadBarCenterItems = ({
  isCompact,
  gitMenuOpen,
  setGitMenuOpen,
  handleGitMenuAction,
  collectionActionMenuOpen,
  setCollectionActionMenuOpen,
  handleCollectionActionMenuAction,
}: HeadBarCenterItemsProps) => {
  return (
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
      <ActionMenu
        items={collectionActionMenuItems}
        trigger={
          <ActionButton
            icon="ThreeVerticalDots"
            iconClassName="text-(--moss-headBar-icon-primary-text)"
            customHoverBackground="hover:bg-[var(--moss-headBar-primary-background-hover)]"
            className="mr-[-4px]"
            title="Collection Actions"
          />
        }
        open={collectionActionMenuOpen}
        onOpenChange={setCollectionActionMenuOpen}
        onSelect={(item) => {
          handleCollectionActionMenuAction(item.id);
        }}
      />
      <Divider />
      <ActionMenu
        items={gitBranchMenuItems}
        trigger={
          <IconLabelButton
            leftIcon="HeadBarGit"
            leftIconClassName="text-(--moss-headBar-icon-primary-text)"
            rightIcon="ChevronDown"
            className="hover:bg-[var(--moss-headBar-primary-background-hover)]"
            title="main"
          />
        }
        open={gitMenuOpen}
        onOpenChange={setGitMenuOpen}
        onSelect={(item) => {
          handleGitMenuAction(item.id);
        }}
      />
    </div>
  );
};

interface HeadBarRightItemsProps {
  isCompact: boolean;
  userMenuOpen: boolean;
  setUserMenuOpen: (open: boolean) => void;
  handleUserMenuAction: (action: string) => void;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  openPanel: (panel: string) => void;
  os: string | null;
}

const HeadBarRightItems = ({
  isCompact,
  userMenuOpen,
  setUserMenuOpen,
  handleUserMenuAction,
  showDebugPanels,
  setShowDebugPanels,
  openPanel,
  os,
}: HeadBarRightItemsProps) => {
  return (
    <div className="flex items-center">
      <ActionMenu
        items={userMenuItems}
        trigger={
          <IconLabelButton
            leftIcon="HeadBarUserAvatar"
            leftIconClassName="text-(--moss-primary) size-4.5"
            rightIcon="ChevronDown"
            title="g10z3r"
            className="mr-2"
            compact={isCompact}
          />
        }
        open={userMenuOpen}
        onOpenChange={setUserMenuOpen}
        onSelect={(item) => {
          handleUserMenuAction(item.id);
        }}
      />

      {os === "macos" && (
        <ModeToggle className="mr-2 border-1 border-[var(--moss-headBar-border-color)]" compact={isCompact} />
      )}

      <CollapsibleActionMenu
        isCompact={isCompact}
        showDebugPanels={showDebugPanels}
        setShowDebugPanels={setShowDebugPanels}
        openPanel={openPanel}
      />
    </div>
  );
};

export const HeadBar = () => {
  const os = type();
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const [isCompact, setIsCompact] = useState(window.innerWidth < COMPACT_MODE_THRESHOLD);
  const [userMenuOpen, setUserMenuOpen] = useState(false);
  const [gitMenuOpen, setGitMenuOpen] = useState(false);
  const [windowsMenuOpen, setWindowsMenuOpen] = useState(false);
  const [collectionActionMenuOpen, setCollectionActionMenuOpen] = useState(false);
  const [workspaceMenuOpen, setWorkspaceMenuOpen] = useState(false);

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

  // Handle user menu actions
  const handleUserMenuAction = (action: string) => {
    console.log(`User action: ${action}`);
    // Here you would handle different user actions like profile, settings, logout, etc.
  };

  // Handle git menu actions
  const handleGitMenuAction = (action: string) => {
    console.log(`Git action: ${action}`);
    // Here you would handle different git actions like branch switching, pull, push, etc.
  };

  // Handle Windows menu actions
  const handleWindowsMenuAction = (action: string) => {
    console.log(`Windows menu action: ${action}`);
    // Here you would handle different Windows menu actions
  };

  // Handle collection action menu actions
  const handleCollectionActionMenuAction = (action: string) => {
    console.log(`Collection action: ${action}`);
    // Here you would handle different collection actions
  };

  // Handle workspace menu actions
  const handleWorkspaceMenuAction = (action: string) => {
    console.log(`Workspace action: ${action}`);
    // Here you would handle different workspace actions
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
          <HeadBarLeftItems
            isCompact={isCompact}
            windowsMenuOpen={windowsMenuOpen}
            setWindowsMenuOpen={setWindowsMenuOpen}
            handleWindowsMenuAction={handleWindowsMenuAction}
            workspaceMenuOpen={workspaceMenuOpen}
            setWorkspaceMenuOpen={setWorkspaceMenuOpen}
            handleWorkspaceMenuAction={handleWorkspaceMenuAction}
            os={os}
          />

          {/*HeadBar Center items*/}
          <HeadBarCenterItems
            isCompact={isCompact}
            gitMenuOpen={gitMenuOpen}
            setGitMenuOpen={setGitMenuOpen}
            handleGitMenuAction={handleGitMenuAction}
            collectionActionMenuOpen={collectionActionMenuOpen}
            setCollectionActionMenuOpen={setCollectionActionMenuOpen}
            handleCollectionActionMenuAction={handleCollectionActionMenuAction}
          />

          {/*HeadBar Right-side items*/}
          <HeadBarRightItems
            isCompact={isCompact}
            userMenuOpen={userMenuOpen}
            setUserMenuOpen={setUserMenuOpen}
            handleUserMenuAction={handleUserMenuAction}
            showDebugPanels={showDebugPanels}
            setShowDebugPanels={setShowDebugPanels}
            openPanel={openPanel}
            os={os}
          />
        </div>
      </div>

      {os !== undefined && os !== "macos" && (os === "windows" || os === "linux") && <Controls os={os} />}
      {os !== undefined && os !== "macos" && os !== "windows" && os !== "linux" && <Controls os={os} />}
    </header>
  );
};
