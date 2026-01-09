import { useState } from "react";

import { useDeleteWorkspace } from "@/adapters/tanstackQuery/workspace";
import { useCurrentWorkspace } from "@/hooks";
import { useModal } from "@/hooks/useModal";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu, ConfirmationModal, IconLabelButton } from "@/workbench/ui/components";
import { NewWorkspaceModal } from "@/workbench/ui/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/workbench/ui/components/Modals/Workspace/OpenWorkspaceModal";
import { renderActionMenuItem } from "@/workbench/utils/renderActionMenuItem";
import { useWorkspaceMenu } from "@/workbench/ui/parts/HeadBar/WorkspaceMenuProvider";
import { HeadBarActionProps, useWorkspaceActions } from "@/workbench/ui/parts/HeadBar/HeadBarActions";

interface SidebarHeaderProps {
  toolbar?: React.ReactNode;
}

export const SidebarHeader = ({ toolbar: toolbar }: SidebarHeaderProps) => {
  const { currentWorkspace } = useCurrentWorkspace();
  const { selectedWorkspaceMenuItems } = useWorkspaceMenu();
  const [isOpen, setIsOpen] = useState(false);

  const { showDebugPanels, setShowDebugPanels, openPanel } = useTabbedPaneStore();
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
    showModal: showDeleteConfirmModal,
    closeModal: closeDeleteConfirmModal,
    openModal: openDeleteConfirmModal,
  } = useModal();

  const actionProps: HeadBarActionProps = {
    openPanel,
    showDebugPanels,
    setShowDebugPanels,
    openNewWorkspaceModal,
    openOpenWorkspaceModal,
    workspaceToDelete,
    setWorkspaceToDelete,
    showDeleteConfirmModal,
    openDeleteConfirmModal,
  };

  const handleWorkspaceMenuAction = useWorkspaceActions(actionProps);

  const handleDeleteWorkspace = () => {
    if (workspaceToDelete) {
      deleteWorkspace(
        { id: workspaceToDelete.id },
        {
          onError: (error) => {
            console.error("Failed to delete workspace:", error.message);
          },
          onSettled: () => {
            setWorkspaceToDelete(null);
            closeDeleteConfirmModal();
          },
        }
      );
    }
  };

  return (
    <>
      <div className="text-(--moss-primary-foreground) relative flex min-h-9 items-center px-2">
        <div className="min-w-0 flex-1 overflow-hidden">
          <ActionMenu.Root open={isOpen} onOpenChange={setIsOpen}>
            <ActionMenu.Trigger asChild>
              <IconLabelButton
                title={currentWorkspace?.name}
                rightIcon="ChevronDown"
                rightIconClassName={cn("size-3.5 transition-transform duration-200", {
                  "rotate-180": isOpen,
                })}
                className="max-w-full px-1.5 py-0.5 text-base font-medium"
              />
            </ActionMenu.Trigger>
            <ActionMenu.Content>
              {selectedWorkspaceMenuItems.map((item) => renderActionMenuItem(item, handleWorkspaceMenuAction))}
            </ActionMenu.Content>
          </ActionMenu.Root>
        </div>
        {toolbar && <div className="relative z-10 flex shrink-0 items-center gap-1 pl-2">{toolbar}</div>}
      </div>

      {showNewWorkspaceModal && (
        <NewWorkspaceModal showModal={showNewWorkspaceModal} closeModal={closeNewWorkspaceModal} />
      )}

      {showOpenWorkspaceModal && (
        <OpenWorkspaceModal showModal={showOpenWorkspaceModal} closeModal={closeOpenWorkspaceModal} />
      )}

      {showDeleteConfirmModal && (
        <ConfirmationModal
          showModal={showDeleteConfirmModal}
          closeModal={closeDeleteConfirmModal}
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
    </>
  );
};
