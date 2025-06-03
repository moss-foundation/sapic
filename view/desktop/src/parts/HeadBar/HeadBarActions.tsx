import { useOpenWorkspace } from "@/hooks/workbench/useOpenWorkspace";
import { useWorkspaceMapping } from "@/hooks/workbench/useWorkspaceMapping";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";
import { RefObject } from "react";

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
  showDeleteConfirmModal?: boolean;
  setShowDeleteConfirmModal?: (show: boolean) => void;
  workspaceToDelete?: { id: string; name: string } | null;
  setWorkspaceToDelete?: (workspace: { id: string; name: string } | null) => void;
  setShowRenameWorkspaceModal?: (show: boolean) => void;
  setWorkspaceToRename?: (workspace: { id: string; name: string } | null) => void;
}

/**
 * User menu action handler
 */
export const useUserMenuActions = (props: HeadBarActionProps) => {
  const { setSelectedUser } = props;

  return (action: string) => {
    console.log(`User action: ${action}`);
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
  };
};

/**
 * Collection action menu handler
 */
export const useCollectionActions = (props: HeadBarActionProps) => {
  const { setCollectionName, collectionButtonRef, setIsRenamingCollection } = props;

  const startRenameCollection = () => {
    setIsRenamingCollection?.(true);

    setTimeout(() => {
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
  const {
    openPanel,
    setShowDebugPanels,
    showDebugPanels,
    openNewWorkspaceModal,
    openOpenWorkspaceModal,
    setShowDeleteConfirmModal,
    setWorkspaceToDelete,
    setShowRenameWorkspaceModal,
    setWorkspaceToRename,
  } = props;

  const { mutate: openWorkspace } = useOpenWorkspace();
  const { getWorkspaceById } = useWorkspaceMapping();
  const activeWorkspace = useActiveWorkspace();

  return (action: string) => {
    console.log(`Workspace action: ${action}`);

    if (action.startsWith("workspace:")) {
      const workspaceId = extractWorkspaceId(action);
      openWorkspace(workspaceId);
      return;
    }

    const workspaceAction = action.match(
      /^([a-zA-Z0-9_-]+)-(rename|duplicate|delete|new|save|save-all|edit-configurations)$/
    );
    if (workspaceAction) {
      const [, workspaceId, actionType] = workspaceAction;
      console.log(`Workspace action for ${workspaceId}: ${actionType}`);

      const generalActions = ["new", "open", "home", "logs", "debug", "separator"];
      if (generalActions.includes(workspaceId)) {
        console.log(`Skipping false match - "${workspaceId}" is a general action keyword`);
      } else {
        if (actionType === "delete") {
          const workspace = getWorkspaceById(workspaceId);
          if (workspace && setShowDeleteConfirmModal && setWorkspaceToDelete) {
            setWorkspaceToDelete({
              id: workspaceId,
              name: workspace.displayName,
            });
            setShowDeleteConfirmModal(true);
          }
          return;
        }

        if (actionType === "rename") {
          const workspace = getWorkspaceById(workspaceId);
          if (workspace && setShowRenameWorkspaceModal && setWorkspaceToRename) {
            // Switch to target workspace first (backend only supports updating active workspace)
            openWorkspace(workspaceId);

            setTimeout(() => {
              setWorkspaceToRename({
                id: workspaceId,
                name: workspace.displayName,
              });
              setShowRenameWorkspaceModal(true);
            }, 100);
          }
          return;
        }

        return;
      }
    }

    if (action === "new-workspace") {
      openNewWorkspaceModal?.();
    } else if (action === "open-workspace") {
      openOpenWorkspaceModal?.();
    } else if (action === "delete") {
      if (activeWorkspace && setShowDeleteConfirmModal && setWorkspaceToDelete) {
        setWorkspaceToDelete({
          id: activeWorkspace.id,
          name: activeWorkspace.displayName,
        });
        setShowDeleteConfirmModal(true);
      }
    } else if (action === "rename") {
      if (activeWorkspace && setShowRenameWorkspaceModal && setWorkspaceToRename) {
        setWorkspaceToRename({
          id: activeWorkspace.id,
          name: activeWorkspace.displayName,
        });
        setShowRenameWorkspaceModal(true);
      }
    } else if (action === "home") {
      openPanel("Home");
    } else if (action === "logs") {
      openPanel("Logs");
    } else if (action === "debug") {
      setShowDebugPanels(!showDebugPanels);
    }
  };
};
