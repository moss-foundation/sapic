import { useState } from "react";

import { ActionMenu, ConfirmationModal } from "@/components";
import ActionButton from "@/components/ActionButton";
import { useDeleteEnvironment, useModal, useStreamEnvironments, useUpdateEnvironment } from "@/hooks";
import { useWorkspaceListStore } from "@/store/workspaceList";
import { cn } from "@/utils";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface GlobalEnvironmentsListActionsProps {
  environment: StreamEnvironmentsEvent;
  setIsEditing: (isEditing: boolean) => void;
}

export const GlobalEnvironmentsListActions = ({ environment, setIsEditing }: GlobalEnvironmentsListActionsProps) => {
  const { setActiveEnvironment, activeEnvironment } = useWorkspaceListStore();

  const { data: environments } = useStreamEnvironments();
  const { mutate: deleteEnvironment } = useDeleteEnvironment();
  const { mutate: updateEnvironment } = useUpdateEnvironment();

  const [showActionMenu, setShowActionMenu] = useState(false);

  const { showModal: showDeleteModal, setShowModal: setShowDeleteModal, closeModal: setHideDeleteModal } = useModal();

  const handleDeleteEnvironment = () => {
    deleteEnvironment({ id: environment.id });

    const environmentsAfterDeleted = environments?.filter(
      (env) => env?.order !== undefined && environment?.order !== undefined && env.order > environment.order
    );

    environmentsAfterDeleted?.forEach((env) => {
      if (env && typeof env.order === "number") {
        updateEnvironment({ id: env.id, order: env.order - 1, varsToAdd: [], varsToUpdate: [], varsToDelete: [] });
      }
    });
  };

  return (
    <div className="z-10 flex items-center gap-1">
      <ActionButton
        onClick={(e) => {
          e.stopPropagation();
          setActiveEnvironment(environment);
        }}
        icon={activeEnvironment?.id === environment.id ? "EnvironmentSelectionActive" : "EnvironmentSelection"}
        className={cn("cursor-pointer group-hover/GlobalEnvironmentsList:opacity-100", {
          "opacity-0": activeEnvironment?.id !== environment.id,
        })}
        customHoverBackground="hover:background-(--moss-gray-10)"
      />

      <ActionMenu.Root onOpenChange={setShowActionMenu} modal={showActionMenu}>
        <ActionMenu.Trigger
          asChild
          className={cn("opacity-0 group-hover/GlobalEnvironmentsList:opacity-100", { "opacity-0": showActionMenu })}
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
          onConfirm={handleDeleteEnvironment}
        />
      )}
    </div>
  );
};
