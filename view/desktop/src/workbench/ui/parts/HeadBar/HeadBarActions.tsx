import { RefObject } from "react";

import { useWorkspaceMapping } from "@/hooks/workbench/derived/useWorkspaceMapping";
import { useCloseWorkspace } from "@/hooks/workbench/useCloseWorkspace";
import { useOpenWorkspace } from "@/hooks/workbench/useOpenWorkspace";
import { useActiveWorkspace } from "@/hooks/workspace/derived/useActiveWorkspace";
import { OpenInTargetEnum } from "@/main/types";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

// Helper to extract workspace ID from prefixed action ID
const extractWorkspaceId = (actionId: string): string => {
  return actionId.startsWith("workspace:") ? actionId.replace("workspace:", "") : actionId;
};

export interface HeadBarActionProps {
  openPanel: (panel: string) => void;
  setShowDebugPanels: (show: boolean) => void;
  showDebugPanels: boolean;
  setProjectName?: (name: string) => void;
  projectButtonRef?: RefObject<HTMLButtonElement>;
  setIsRenamingProject?: (isRenaming: boolean) => void;
  setSelectedUser?: (user: string | null) => void;
  setSelectedBranch?: (branch: string | null) => void;
  openNewWorkspaceModal?: () => void;
  openOpenWorkspaceModal?: () => void;
  showDeleteConfirmModal?: boolean;
  workspaceToDelete?: { id: string; name: string } | null;
  setWorkspaceToDelete?: (workspace: { id: string; name: string } | null) => void;
  openDeleteConfirmModal?: () => void;
}

/**
 * User menu action handler
 */
export const useUserMenuActions = (props: HeadBarActionProps) => {
  const { setSelectedUser } = props;

  return (action: string) => {
    if (action === "sign-in" || action === "user-profile") {
      setSelectedUser?.("Selected User");
    }
    return;
  };
};

/**
 * Git menu action handler
 */
export const useGitMenuActions = (props: HeadBarActionProps) => {
  const { setSelectedBranch } = props;

  return (action: string) => {
    if (action === "main" || action === "master") {
      setSelectedBranch?.(action);
    }
    return;
  };
};

/**
 * Windows menu action handler
 */
export const useWindowsMenuActions = () => {
  return (action: string) => {
    console.log(`Windows menu action: ${action}`);
    return;
  };
};

/**
 * Project action menu handler
 */
export const useProjectActions = (props: HeadBarActionProps) => {
  const {
    setProjectName: setProjectName,
    projectButtonRef: projectButtonRef,
    setIsRenamingProject: setIsRenamingProject,
  } = props;

  const startRenameProject = () => {
    setIsRenamingProject?.(true);

    setTimeout(() => {
      if (projectButtonRef?.current) {
        const doubleClickEvent = new MouseEvent("dblclick", {
          bubbles: true,
          cancelable: true,
          view: window,
        });
        projectButtonRef.current.dispatchEvent(doubleClickEvent);
      }
    }, 50);
  };

  const handleRenameProject = (newName: string) => {
    if (newName.trim() !== "") {
      setProjectName?.(newName);
    }
    setIsRenamingProject?.(false);
  };

  const handleProjectActionMenuAction = (action: string) => {
    if (action === "rename") {
      startRenameProject();
    }
    return;
  };

  return {
    handleProjectActionMenuAction,
    handleRenameProject,
    startRenameProject,
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
    setWorkspaceToDelete,
    openDeleteConfirmModal,
  } = props;

  const { mutate: openWorkspace } = useOpenWorkspace();
  const { mutate: closeWorkspace } = useCloseWorkspace();
  const { getWorkspaceById } = useWorkspaceMapping();
  const { activeWorkspace, activeWorkspaceId } = useActiveWorkspace();

  const { addOrFocusPanel } = useTabbedPaneStore();

  return (action: string) => {
    if (action.startsWith("workspace:")) {
      const workspaceId = extractWorkspaceId(action);
      openWorkspace({ id: workspaceId, openInTarget: OpenInTargetEnum.CURRENT_WINDOW });
      return;
    }

    const workspaceAction = action.match(
      /^([a-zA-Z0-9_-]+)-(rename|duplicate|delete|new|save|save-all|edit-configurations)$/
    );
    if (workspaceAction) {
      const [, workspaceId, actionType] = workspaceAction;

      const generalActions = ["new", "open", "kitchensink", "logs", "debug", "separator"];
      if (generalActions.includes(workspaceId)) {
        console.log(`Skipping false match - "${workspaceId}" is a general action keyword`);
        return;
      } else {
        if (actionType === "delete") {
          const workspace = getWorkspaceById(workspaceId);
          if (workspace) {
            setWorkspaceToDelete?.({
              id: workspaceId,
              name: workspace.name,
            });
            openDeleteConfirmModal?.();
          }
          return;
        }

        if (actionType === "rename") {
          if (workspaceId === activeWorkspaceId) {
            addOrFocusPanel({
              id: "WorkspaceSettings",
              title: activeWorkspace?.name || "Workspace Settings",
              component: "WorkspaceSettingsView",
              params: {
                tabIcon: "Workspace",
                workspace: true,
              },
            });
            return;
          }

          const workspace = getWorkspaceById(workspaceId);
          if (workspace) {
            openWorkspace(
              { id: workspaceId, openInTarget: OpenInTargetEnum.CURRENT_WINDOW },
              {
                onSuccess: () => {
                  addOrFocusPanel({
                    id: "WorkspaceSettings",
                    title: workspace.name,
                    component: "WorkspaceSettingsView",
                    params: {
                      tabIcon: "Workspace",
                    },
                  });
                },
                onError: (error) => {
                  console.error("Failed to open workspace:", error.message);
                },
              }
            );
          } else {
            console.error(`Workspace not found for ID: ${workspaceId}`);
          }
          return;
        }

        return;
      }
    }

    switch (action) {
      case "new-workspace":
        openNewWorkspaceModal?.();
        break;
      case "open-workspace":
        openOpenWorkspaceModal?.();
        break;
      case "delete":
        if (activeWorkspace) {
          setWorkspaceToDelete?.({
            id: activeWorkspace.id,
            name: activeWorkspace.name,
          });
          openDeleteConfirmModal?.();
        }
        break;
      case "rename":
        if (activeWorkspace) {
          addOrFocusPanel({
            id: "WorkspaceSettings",
            title: activeWorkspace.name,
            component: "WorkspaceSettingsView",
            params: {
              tabIcon: "Workspace",
            },
          });
        }
        break;
      case "kitchensink":
        openPanel("KitchenSink");
        break;
      case "logs":
        openPanel("Logs");
        break;
      case "debug":
        setShowDebugPanels(!showDebugPanels);
        break;
      case "exit-workspace":
        if (activeWorkspace) {
          closeWorkspace(activeWorkspace.id);
        }
        break;
      default:
        console.log(`Unhandled workspace action: ${action}`);
        break;
    }
  };
};
