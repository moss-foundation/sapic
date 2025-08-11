import { useState } from "react";

import { ActionMenu, ConfirmationModal } from "@/components";
import ActionButton from "@/components/ActionButton";
import { useModal } from "@/hooks";
import { useDeleteEnvironment } from "@/hooks/environment";
import { useWorkspaceListStore } from "@/store/workspaceList";
import { cn } from "@/utils";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface WorkspaceListItemActionsProps {
  environment: StreamEnvironmentsEvent;
  setIsEditing: (isEditing: boolean) => void;
}

export const WorkspaceListItemActions = ({ environment, setIsEditing }: WorkspaceListItemActionsProps) => {
  const { setActiveEnvironment, activeEnvironment } = useWorkspaceListStore();
  const { mutate: deleteEnvironment } = useDeleteEnvironment();

  const [showActionMenu, setShowActionMenu] = useState(false);

  const { showModal: showDeleteModal, setShowModal: setShowDeleteModal, closeModal: setHideDeleteModal } = useModal();

  return (
    <div className="z-10 flex items-center gap-1">
      <ActionButton
        onClick={() => setActiveEnvironment(environment)}
        icon={activeEnvironment?.id === environment.id ? "EnvironmentSelectionActive" : "EnvironmentSelection"}
        className={cn("cursor-pointer group-hover/WorkspaceListItem:opacity-100", {
          "opacity-0": activeEnvironment?.id !== environment.id,
        })}
        customHoverBackground="hover:background-(--moss-gray-10)"
      />

      <ActionMenu.Root onOpenChange={setShowActionMenu} modal={showActionMenu}>
        <ActionMenu.Trigger
          asChild
          className={cn("opacity-0 group-hover/WorkspaceListItem:opacity-100", { "opacity-0": showActionMenu })}
        >
          <ActionButton
            icon="MoreHorizontal"
            className="cursor-pointer"
            customHoverBackground="hover:background-(--moss-gray-10)"
          />
        </ActionMenu.Trigger>

        <ActionMenu.Portal>
          <ActionMenu.Content>
            <ActionMenu.Item onClick={() => setIsEditing(true)}>Edit</ActionMenu.Item>
            <ActionMenu.Item onClick={() => setShowDeleteModal(true)}>Delete</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>

      {showDeleteModal && (
        <ConfirmationModal
          showModal={showDeleteModal}
          closeModal={setHideDeleteModal}
          title="Delete Environment"
          message={`Are you sure you want to delete ${environment.name} environment?`}
          onConfirm={() => {
            deleteEnvironment({ id: environment.id });
          }}
        />
      )}
    </div>
  );
};
