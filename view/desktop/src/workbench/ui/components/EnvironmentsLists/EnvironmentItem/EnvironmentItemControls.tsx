import { MouseEvent, useState } from "react";

import { useDeleteEnvironment } from "@/adapters";
import { useActivateEnvironment } from "@/adapters/tanstackQuery/environment/useActivateEnvironment";
import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { useCurrentWorkspace, useModal } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { useBatchPutEnvironmentItemState } from "@/workbench/adapters/tanstackQuery/environmentItemState/useBatchPutEnvironmentItemState";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu, ConfirmationModal } from "@/workbench/ui/components";
import ActionButton from "@/workbench/ui/components/ActionButton";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";

interface EnvironmentItemControlsProps {
  environment: EnvironmentSummary;
  setIsEditing: (isEditing: boolean) => void;
  instruction: Instruction | null;
  type: ENVIRONMENT_ITEM_DRAG_TYPE;
}

export const EnvironmentItemControls = ({
  environment,
  setIsEditing,
  instruction,
  type,
}: EnvironmentItemControlsProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { activePanelId } = useTabbedPaneStore();

  const { workspaceEnvironments } = useGetWorkspaceEnvironments();

  const { mutate: activateEnvironment } = useActivateEnvironment();
  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: batchPutEnvironmentItemState } = useBatchPutEnvironmentItemState();
  const { showModal: showDeleteModal, setShowModal: setShowDeleteModal, closeModal: closeDeleteModal } = useModal();

  const [showActionMenu, setShowActionMenu] = useState(false);

  const handleDeleteEnvironment = async () => {
    await deleteEnvironment({ id: environment.id, projectId: environment.projectId ?? undefined });

    const environmentsAfterDeleted = workspaceEnvironments?.filter((env) => env.order! > environment.order!);

    if (!environmentsAfterDeleted || environmentsAfterDeleted.length === 0) return;

    await batchPutEnvironmentItemState({
      environmentItemStates: environmentsAfterDeleted.map((env) => ({
        id: env.id,
        order: (env.order ?? 0) - 1,
      })),
      workspaceId: currentWorkspaceId,
    });
  };

  const handleSetActiveEnvironment = (e: MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    if (environment.projectId) {
      activateEnvironment({ environmentId: environment.id, projectId: environment.projectId });
    } else {
      activateEnvironment({ environmentId: environment.id });
    }
  };

  const handleDeleteEnvironmentClick = async () => {
    await handleDeleteEnvironment();
    closeDeleteModal();
  };

  return (
    <>
      <Tree.NodeControls
        isActive={activePanelId === environment.id}
        instruction={instruction}
        hideDragHandle
        depth={type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ? 0 : 1}
        dropIndicatorFullWidth
      >
        <Tree.NodeTriggers className="cursor-pointer overflow-hidden">
          <Icon icon={type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ? "ProjectEnvironment" : "WorkspaceEnvironment"} />
          <Tree.NodeLabel label={environment.name} />
          <Tree.NodeDirCount count={environment.totalVariables} />
        </Tree.NodeTriggers>

        <Tree.NodeActions>
          <Tree.ActionsHover invisible={true} forceVisible={environment.isActive}>
            <ActionButton
              onClick={handleSetActiveEnvironment}
              icon={environment.isActive ? "EnvironmentSelectionActive" : "EnvironmentSelection"}
              hoverVariant="list"
            />
          </Tree.ActionsHover>
          <Tree.ActionsHover invisible={true} forceVisible={showActionMenu}>
            <ActionMenu.Root onOpenChange={setShowActionMenu} modal={showActionMenu}>
              <ActionMenu.Trigger asChild>
                <ActionButton icon="MoreHorizontal" className="cursor-pointer" hoverVariant="list" />
              </ActionMenu.Trigger>

              <ActionMenu.Portal>
                <ActionMenu.Content>
                  <ActionMenu.Item
                    onClick={(e) => {
                      e.stopPropagation();
                      setIsEditing(true);
                    }}
                  >
                    Edit
                  </ActionMenu.Item>
                  <ActionMenu.Item
                    onClick={(e) => {
                      e.stopPropagation();
                      setShowDeleteModal(true);
                    }}
                  >
                    Delete
                  </ActionMenu.Item>
                </ActionMenu.Content>
              </ActionMenu.Portal>
            </ActionMenu.Root>
          </Tree.ActionsHover>
        </Tree.NodeActions>
      </Tree.NodeControls>

      {showDeleteModal && (
        <ConfirmationModal
          showModal={showDeleteModal}
          closeModal={closeDeleteModal}
          title="Delete Environment"
          message={`Are you sure you want to delete ${environment.name} environment?`}
          onConfirm={handleDeleteEnvironmentClick}
        />
      )}
    </>
  );
};
