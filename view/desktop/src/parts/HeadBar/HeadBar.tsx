import { ActionButton, Divider, IconLabelButton } from "@/components";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";
import { useState } from "react";
import { useResponsive } from "@/hooks/useResponsive";

import { Controls } from "./Controls/Controls";
import { ModeToggle } from "./ModeToggle";
import ActionMenu from "@/components/ActionMenu/ActionMenu";
import CollapsibleActionMenu from "./CollapsibleActionMenu";
import { userMenuItems, gitBranchMenuItems, windowsMenuItems } from "./mockHeadBarData";
import { collectionActionMenuItems, workspaceMenuItems } from "./HeadBarData";

interface HeadBarLeftItemsProps {
  isLarge: boolean;
  breakpoint: string;
  windowsMenuOpen: boolean;
  setWindowsMenuOpen: (open: boolean) => void;
  handleWindowsMenuAction: (action: string) => void;
  workspaceMenuOpen: boolean;
  setWorkspaceMenuOpen: (open: boolean) => void;
  handleWorkspaceMenuAction: (action: string) => void;
  os: string | null;
}

const HeadBarLeftItems = ({
  isLarge,
  breakpoint,
  windowsMenuOpen,
  setWindowsMenuOpen,
  handleWindowsMenuAction,
  workspaceMenuOpen,
  setWorkspaceMenuOpen,
  handleWorkspaceMenuAction,
  os,
}: HeadBarLeftItemsProps) => {
  const isWindowsOrLinux = os === "windows" || os === "linux";

  return (
    <div
      className={cn("flex items-center", {
        "gap-0.5": breakpoint === "md",
        "gap-1.5": ["lg", "xl", "2xl"].includes(breakpoint),
      })}
      data-tauri-drag-region
    >
      {isWindowsOrLinux && (
        <>
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
          <ModeToggle className="mr-0.5 border-1 border-[var(--moss-headBar-border-color)]" compact={isLarge} />
        </>
      )}
      <ActionMenu
        items={workspaceMenuItems}
        trigger={
          <IconLabelButton rightIcon="ChevronDown" title="My Workspace" labelClassName="text-md" className="h-[24px]" />
        }
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
        className="h-[24px]"
        compact={isLarge}
      />
    </div>
  );
};

interface HeadBarCenterItemsProps {
  isMedium: boolean;
  isXLarge: boolean;
  breakpoint: string;
  gitMenuOpen: boolean;
  setGitMenuOpen: (open: boolean) => void;
  handleGitMenuAction: (action: string) => void;
  collectionActionMenuOpen: boolean;
  setCollectionActionMenuOpen: (open: boolean) => void;
  handleCollectionActionMenuAction: (action: string) => void;
}

const HeadBarCenterItems = ({
  isMedium,
  isXLarge,
  gitMenuOpen,
  setGitMenuOpen,
  handleGitMenuAction,
  collectionActionMenuOpen,
  setCollectionActionMenuOpen,
  handleCollectionActionMenuAction,
}: HeadBarCenterItemsProps) => {
  return (
    <div
      className={cn(
        "flex h-[26px] items-center rounded border border-[var(--moss-headBar-border-color)] bg-[var(--moss-headBar-primary-background)] px-0.5",
        isXLarge ? "" : "absolute left-1/2 -translate-x-1/2 transform"
      )}
      data-tauri-drag-region
    >
      <IconLabelButton
        leftIcon="HeadBarCollection"
        leftIconClassName="text-(--moss-headBar-icon-primary-text)"
        className={
          isMedium
            ? "mr-[3px] h-[22px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
            : "mr-[30px] h-[22px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
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
            className="ml-[-2px] h-[22px] hover:bg-[var(--moss-headBar-primary-background-hover)]"
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
  isMedium: boolean;
  isLarge: boolean;
  breakpoint: string;
  userMenuOpen: boolean;
  setUserMenuOpen: (open: boolean) => void;
  handleUserMenuAction: (action: string) => void;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  openPanel: (panel: string) => void;
  os: string | null;
}

const HeadBarRightItems = ({
  isMedium,
  isLarge,
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
        align="end"
        trigger={
          <IconLabelButton
            leftIcon="HeadBarUserAvatar"
            leftIconClassName="text-(--moss-primary) size-4.5"
            rightIcon="ChevronDown"
            title="g10z3r"
            className="mr-2 h-[24px]"
            compact={isMedium}
          />
        }
        open={userMenuOpen}
        onOpenChange={setUserMenuOpen}
        onSelect={(item) => {
          handleUserMenuAction(item.id);
        }}
      />

      {os === "macos" && (
        <ModeToggle className="mr-2 border-1 border-[var(--moss-headBar-border-color)]" compact={isLarge} />
      )}

      <CollapsibleActionMenu
        isCompact={isMedium}
        showDebugPanels={showDebugPanels}
        setShowDebugPanels={setShowDebugPanels}
        openPanel={openPanel}
      />
    </div>
  );
};

export const HeadBar = () => {
  const os = "macos";
  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const { isMedium, isLarge, isXLarge, breakpoint } = useResponsive();
  const [userMenuOpen, setUserMenuOpen] = useState(false);
  const [gitMenuOpen, setGitMenuOpen] = useState(false);
  const [windowsMenuOpen, setWindowsMenuOpen] = useState(false);
  const [collectionActionMenuOpen, setCollectionActionMenuOpen] = useState(false);
  const [workspaceMenuOpen, setWorkspaceMenuOpen] = useState(false);

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
    // Handle different workspace actions
    if (action === "home") openPanel("Home");
    if (action === "logs") openPanel("Logs");
    if (action === "debug") setShowDebugPanels(!showDebugPanels);
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
            isLarge={isLarge}
            breakpoint={breakpoint}
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
            isMedium={isMedium}
            isXLarge={isXLarge}
            breakpoint={breakpoint}
            gitMenuOpen={gitMenuOpen}
            setGitMenuOpen={setGitMenuOpen}
            handleGitMenuAction={handleGitMenuAction}
            collectionActionMenuOpen={collectionActionMenuOpen}
            setCollectionActionMenuOpen={setCollectionActionMenuOpen}
            handleCollectionActionMenuAction={handleCollectionActionMenuAction}
          />

          {/*HeadBar Right-side items*/}
          <HeadBarRightItems
            isMedium={isMedium}
            isLarge={isLarge}
            breakpoint={breakpoint}
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
