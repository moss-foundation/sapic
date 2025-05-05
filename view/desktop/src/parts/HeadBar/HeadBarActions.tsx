import { RefObject } from "react";
import { useWorkspaceContext } from "@/context/WorkspaceContext";

export interface HeadBarActionProps {
  openPanel: (panel: string) => void;
  setShowDebugPanels: (show: boolean) => void;
  showDebugPanels: boolean;
  setCollectionName?: (name: string) => void;
  collectionButtonRef?: RefObject<HTMLButtonElement>;
  setIsRenamingCollection?: (isRenaming: boolean) => void;
  setSelectedWorkspace?: (workspace: string | null) => void;
  setSelectedUser?: (user: string | null) => void;
  setSelectedBranch?: (branch: string | null) => void;
  openNewWorkspaceModal?: () => void;
  openOpenWorkspaceModal?: () => void;
}

/**
 * User menu action handler
 */
export const useUserMenuActions = (props: HeadBarActionProps) => {
  const { setSelectedUser } = props;

  return (action: string) => {
    console.log(`User action: ${action}`);
    // Here you would handle different user actions like profile, settings, logout, etc.
    if (action === "sign-in" || action === "user-profile") {
      setSelectedUser?.("Selected User");
    }
  };
};

/**
 * Git menu action handler
 */
export const useGitMenuActions = (props: HeadBarActionProps) => {
  const { setSelectedBranch } = props;

  return (action: string) => {
    console.log(`Git action: ${action}`);
    // Here you would handle different git actions like branch switching, pull, push, etc.
    if (action === "main" || action === "master") {
      setSelectedBranch?.(action);
    }
  };
};

/**
 * Windows menu action handler
 */
export const useWindowsMenuActions = (props: HeadBarActionProps) => {
  return (action: string) => {
    console.log(`Windows menu action: ${action}`);
    // Here you would handle different Windows menu actions
  };
};

/**
 * Collection action menu handler
 */
export const useCollectionActions = (props: HeadBarActionProps) => {
  const { setCollectionName, collectionButtonRef, setIsRenamingCollection } = props;

  const startRenameCollection = () => {
    setIsRenamingCollection?.(true);

    // Use a small timeout to ensure the menu has closed
    setTimeout(() => {
      // Dispatch a double-click event to the collection button to trigger renaming
      if (collectionButtonRef?.current) {
        const doubleClickEvent = new MouseEvent("dblclick", {
          bubbles: true,
          cancelable: true,
          view: window,
        });
        collectionButtonRef.current.dispatchEvent(doubleClickEvent);
      }
    }, 50);
  };

  const handleRenameCollection = (newName: string) => {
    if (newName.trim() !== "") {
      setCollectionName?.(newName);
    }
    setIsRenamingCollection?.(false);
  };

  const handleCollectionActionMenuAction = (action: string) => {
    console.log(`Collection action: ${action}`);

    // Check if the rename action was selected
    if (action === "rename") {
      startRenameCollection();
    }
  };

  return {
    handleCollectionActionMenuAction,
    handleRenameCollection,
    startRenameCollection,
  };
};

/**
 * Workspace menu action handler
 */
export const useWorkspaceActions = (props: HeadBarActionProps) => {
  const { openPanel, setShowDebugPanels, showDebugPanels, openNewWorkspaceModal, openOpenWorkspaceModal } = props;

  // Use the context for opening workspaces
  const { openAndSelectWorkspace } = useWorkspaceContext();

  return (action: string) => {
    console.log(`Workspace action: ${action}`);

    // Handle workspace selection - now handles any workspace name and opens it
    if (
      action !== "home" &&
      action !== "logs" &&
      action !== "debug" &&
      action !== "new-workspace" &&
      action !== "open-workspace" &&
      !action.startsWith("separator") &&
      !action.startsWith("new-") &&
      !action.startsWith("import-") &&
      !action.startsWith("save") &&
      !action.startsWith("edit-")
    ) {
      // Use the context function that handles both opening and selecting
      openAndSelectWorkspace(action);
    }

    // Handle new workspace modal
    if (action === "new-workspace") {
      openNewWorkspaceModal?.();
    }

    // Handle open workspace modal
    if (action === "open-workspace") {
      openOpenWorkspaceModal?.();
    }

    // Handle different workspace actions
    if (action === "home") openPanel("Home");
    if (action === "logs") openPanel("Logs");
    if (action === "debug") setShowDebugPanels(!showDebugPanels);
  };
};
