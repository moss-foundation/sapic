import { RefObject } from "react";
import { useOpenWorkspace } from "@/hooks/workbench/useOpenWorkspace";

// Helper to extract workspace ID from prefixed action ID
const extractWorkspaceId = (actionId: string): string => {
  return actionId.startsWith("workspace:") ? actionId.replace("workspace:", "") : actionId;
};

export interface HeadBarActionProps {
  openPanel: (panel: string) => void;
  setShowDebugPanels: (show: boolean) => void;
  showDebugPanels: boolean;
  setCollectionName?: (name: string) => void;
  collectionButtonRef?: RefObject<HTMLButtonElement>;
  setIsRenamingCollection?: (isRenaming: boolean) => void;
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

  // Use the hook directly instead of context
  const { mutate: openWorkspace } = useOpenWorkspace();

  return (action: string) => {
    console.log(`Workspace action: ${action}`);

    // Handle opening workspace when the action ID has the workspace: prefix
    if (action.startsWith("workspace:")) {
      const workspaceId = extractWorkspaceId(action);
      openWorkspace(workspaceId);
      return;
    }

    // Handle menu item for a specific workspace
    // e.g., "workspaceId-rename" for renaming a workspace
    const workspaceAction = action.match(/^(.+?)-(rename|duplicate|delete|new|save|edit-.+)$/);
    if (workspaceAction) {
      const [, workspaceId, actionType] = workspaceAction;
      console.log(`Workspace action for ${workspaceId}: ${actionType}`);

      // Here you can implement specific actions for workspace items
      // For example:
      // if (actionType === "rename") { handleRenameWorkspace(workspaceId); }

      return;
    }

    // Handle other specific actions
    if (action === "new-workspace") {
      openNewWorkspaceModal?.();
    } else if (action === "open-workspace") {
      openOpenWorkspaceModal?.();
    } else if (action === "home") {
      openPanel("Home");
    } else if (action === "logs") {
      openPanel("Logs");
    } else if (action === "debug") {
      setShowDebugPanels(!showDebugPanels);
    }

    // Other actions like "rename", "duplicate", etc. will be handled elsewhere
  };
};
