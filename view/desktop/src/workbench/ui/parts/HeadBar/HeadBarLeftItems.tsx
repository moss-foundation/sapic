import { useState } from "react";

import { useDeleteWorkspace } from "@/adapters/tanstackQuery/workspace";
import { useCurrentWorkspace } from "@/hooks";
import { useModal } from "@/hooks/useModal";
import Icon from "@/lib/ui/Icon";
import { cn } from "@/utils";
import { ActionMenu, ConfirmationModal, IconLabelButton } from "@/workbench/ui/components";
import { NewWorkspaceModal } from "@/workbench/ui/components/Modals/Workspace/NewWorkspaceModal";
import { OpenWorkspaceModal } from "@/workbench/ui/components/Modals/Workspace/OpenWorkspaceModal";
import { renderActionMenuItem } from "@/workbench/utils/renderActionMenuItem";

import { useWorkspaceActions } from "./HeadBarActions";
import { windowsMenuItems } from "./mockHeadBarData";
import { useWorkspaceMenu } from "./WorkspaceMenuProvider";

export interface HeadBarLeftItemsProps {
  handleWindowsMenuAction: (action: string) => void;
  os: string | null;
}

export const HeadBarLeftItems = ({ handleWindowsMenuAction, os }: HeadBarLeftItemsProps) => {
  const isWindowsOrLinux = os === "windows" || os === "linux";
  const { currentWorkspace } = useCurrentWorkspace();
  const { selectedWorkspaceMenuItems } = useWorkspaceMenu();
  const [isOpen, setIsOpen] = useState(false);

  // Workspace modals state
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

  const workspaceModals = {
    openNewWorkspaceModal,
    openOpenWorkspaceModal,
    openDeleteConfirmModal: (workspace: { id: string; name: string }) => {
      setWorkspaceToDelete(workspace);
      openDeleteConfirmModal();
    },
  };

  const handleWorkspaceMenuAction = useWorkspaceActions(undefined, workspaceModals);

  const firstLetter = currentWorkspace?.name ? currentWorkspace.name.charAt(0).toUpperCase() : "";

  return (
    <div className={cn("flex items-center justify-start gap-[6px] overflow-hidden")} data-tauri-drag-region>
      {isWindowsOrLinux && (
        <ActionMenu.Root>
          <ActionMenu.Trigger className="hover:background-(--moss-toolbarItem-background-hover) rounded p-1">
            <Icon icon="WindowsMenu" className="size-4.5 cursor-pointer" />
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            {windowsMenuItems.map((item) => renderActionMenuItem(item, handleWindowsMenuAction))}
          </ActionMenu.Content>
        </ActionMenu.Root>
      )}

      <ActionMenu.Root open={isOpen} onOpenChange={setIsOpen}>
        <ActionMenu.Trigger asChild>
          <IconLabelButton
            title={currentWorkspace?.name}
            monogram={firstLetter}
            rightIcon="ChevronDown"
            rightIconClassName={cn("size-3.5 transition-transform duration-200", {
              "rotate-180": isOpen,
            })}
            className="background-(--moss-secondary-background) max-w-full py-0.5 pr-1.5 text-base"
          />
        </ActionMenu.Trigger>
        <ActionMenu.Content>
          {selectedWorkspaceMenuItems.map((item) => renderActionMenuItem(item, handleWorkspaceMenuAction))}
        </ActionMenu.Content>
      </ActionMenu.Root>

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
    </div>
  );
};
