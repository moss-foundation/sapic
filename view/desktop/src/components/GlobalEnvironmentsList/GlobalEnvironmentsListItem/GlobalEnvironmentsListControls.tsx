import { useState } from "react";

import { ActionMenu } from "@/components";
import ActionButton from "@/components/ActionButton";
import { ConfirmationModal } from "@/components/Modals/ConfirmationModal";
import { useDeleteEnvironment, useModal, useStreamEnvironments, useUpdateEnvironment } from "@/hooks";
import { useActivateEnvironment } from "@/hooks/workspace/environment/useActivateEnvironment";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface GlobalEnvironmentsListControlsProps {
  environment: StreamEnvironmentsEvent;
  setIsEditing: (isEditing: boolean) => void;
}

export const GlobalEnvironmentsListControls = ({ environment, setIsEditing }: GlobalEnvironmentsListControlsProps) => {
  const { globalEnvironments } = useStreamEnvironments();
  const { mutate: deleteEnvironment } = useDeleteEnvironment();
  const { mutate: updateEnvironment } = useUpdateEnvironment();
  const { mutate: activateEnvironment } = useActivateEnvironment();

  const { addOrFocusPanel } = useTabbedPaneStore();

  const [showActionMenu, setShowActionMenu] = useState(false);

  const { showModal: showDeleteModal, setShowModal: setShowDeleteModal, closeModal: setHideDeleteModal } = useModal();

  const handleDeleteEnvironment = () => {
    deleteEnvironment({ id: environment.id });

    const environmentsAfterDeleted = globalEnvironments.filter(
      (env) => env?.order !== undefined && environment?.order !== undefined && env.order > environment.order
    );

    environmentsAfterDeleted?.forEach((env) => {
      if (env && typeof env.order === "number") {
        updateEnvironment({ id: env.id, order: env.order - 1, varsToAdd: [], varsToUpdate: [], varsToDelete: [] });
      }
    });
  };

  const handleSetActiveEnvironment = () => {
    activateEnvironment({ environmentId: environment.id });
  };

  const onClick = () => {
    addOrFocusPanel({
      id: environment.id,
      component: "Default",
      title: environment.name,
      params: {
        iconType: "Environment",
      },
    });
  };

  return (
    <>
      <Tree.RootNodeControls>
        <Tree.RootNodeTriggers className="cursor-pointer overflow-hidden" onClick={onClick}>
          <Icon icon="Environment" />
          <span className="truncate">{environment.name}</span>
          <span className="text-(--moss-secondary-text)">({environment.totalVariables})</span>
        </Tree.RootNodeTriggers>

        <Tree.RootNodeActions>
          <Tree.ActionsHover invisible={true} forceVisible={environment.isActive}>
            <ActionButton
              onClick={handleSetActiveEnvironment}
              icon={environment.isActive ? "EnvironmentSelectionActive" : "EnvironmentSelection"}
              customHoverBackground="hover:background-(--moss-gray-10)"
            />
          </Tree.ActionsHover>
          <Tree.ActionsHover invisible={true} forceVisible={showActionMenu}>
            <ActionMenu.Root onOpenChange={setShowActionMenu} modal={showActionMenu}>
              <ActionMenu.Trigger asChild>
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
          </Tree.ActionsHover>
        </Tree.RootNodeActions>
      </Tree.RootNodeControls>

      {showDeleteModal && (
        <ConfirmationModal
          showModal={showDeleteModal}
          closeModal={setHideDeleteModal}
          title="Delete Environment"
          message={`Are you sure you want to delete ${environment.name} environment?`}
          onConfirm={handleDeleteEnvironment}
        />
      )}
    </>
  );
};
