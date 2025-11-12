import { useState } from "react";

import { ConfirmationModal } from "@/components";
import { NewWorkspaceModal } from "@/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/components/Modals/Workspace/OpenWorkspaceModal";
import { useModal } from "@/hooks/useModal";
import { useDeleteWorkspace } from "@/hooks/workbench";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { type } from "@tauri-apps/plugin-os";

import { Controls } from "./Controls/Controls";
import { HeadBarActionProps, useWindowsMenuActions, useWorkspaceActions } from "./HeadBarActions";
import { HeadBarLeftItems } from "./HeadBarLeftItems";
import { HeadBarRightItems } from "./HeadBarRightItems";
import { WorkspaceMenuProvider } from "./WorkspaceMenuProvider";

export const HeadBar = () => {
  const os = type();

  const { showDebugPanels, setShowDebugPanels } = useTabbedPaneStore();
  const { openPanel } = useTabbedPaneStore();
  const { mutate: deleteWorkspace, isPending: isDeleting } = useDeleteWorkspace();
  const [workspaceToDelete, setWorkspaceToDelete] = useState<{ id: string; name: string } | null>(null);

  const {
    showModal: showNewWorkspaceModal,
    closeModal: closeNewWorkspaceModal,
    openModal: openNewWorkspaceModal,
  } = useModal();

  const {
    showModal: showOpenWorkspaceModal,
    closeModal: closeOpenWorkspaceModal,
    openModal: openOpenWorkspaceModal,
  } = useModal();

  const {
    showModal: showDeleteWorkspaceModal,
    closeModal: closeDeleteWorkspaceModal,
    openModal: openDeleteWorkspaceModal,
  } = useModal();

  const actionProps: HeadBarActionProps = {
    openPanel,
    showDebugPanels,
    setShowDebugPanels,
    openNewWorkspaceModal,
    openOpenWorkspaceModal,
    workspaceToDelete,
    setWorkspaceToDelete,
    showDeleteWorkspaceModal,
    openDeleteWorkspaceModal,
    closeDeleteWorkspaceModal,
  };

  const workspaceActionProps: HeadBarActionProps = { ...actionProps };

  const handleWindowsMenuAction = useWindowsMenuActions();
  const handleWorkspaceMenuAction = useWorkspaceActions(workspaceActionProps);

  const handleDeleteWorkspace = () => {
    if (workspaceToDelete) {
      deleteWorkspace(
        { id: workspaceToDelete.id },
        {
          onSuccess: () => {
            setWorkspaceToDelete(null);
            closeDeleteWorkspaceModal();
          },
          onError: (error) => {
            console.error("Failed to delete workspace:", error.message);
            setWorkspaceToDelete(null);
            closeDeleteWorkspaceModal();
          },
        }
      );
    }
  };

  return (
    <WorkspaceMenuProvider>
      <header
        data-tauri-drag-region
        className={cn(
          "header background-(--moss-secondary-background) border-(--moss-border) flex h-full w-screen items-center justify-between border-b"
        )}
      >
        {os === "macos" && <Controls os={os} />}

        <div
          className={cn("relative flex h-full w-full items-center justify-between overflow-clip", {
            "mr-2 pl-2.5 pr-[4px]": os === "macos",
            "ml-[7px]": os === "windows" || os === "linux",
          })}
          style={{ overflowClipMargin: 4 }}
          data-tauri-drag-region
        >
          <HeadBarLeftItems
            handleWindowsMenuAction={handleWindowsMenuAction}
            handleWorkspaceMenuAction={handleWorkspaceMenuAction}
            os={os}
          />

          <HeadBarRightItems os={os} />
        </div>
      </header>

      {showNewWorkspaceModal && (
        <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      )}

      {showOpenWorkspaceModal && (
        <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      )}

      {showDeleteWorkspaceModal && (
        <ConfirmationModal
          showModal={showDeleteWorkspaceModal}
          closeModal={closeDeleteWorkspaceModal}
          title="Delete"
          message={`Delete "${workspaceToDelete?.name}"?`}
          description="This will delete the monitors, scheduled runs and integrations and deactivate the mock servers associated with projects in the workspace."
          confirmLabel={isDeleting ? "Deleting..." : "Delete"}
          cancelLabel="Close"
          onConfirm={handleDeleteWorkspace}
          variant="danger"
          loading={isDeleting}
        />
      )}
    </WorkspaceMenuProvider>
  );
};
