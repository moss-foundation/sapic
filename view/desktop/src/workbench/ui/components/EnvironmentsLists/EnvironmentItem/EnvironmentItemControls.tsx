import { MouseEvent, useState } from "react";

import { useActivateEnvironment } from "@/adapters/tanstackQuery/environment/useActivateEnvironment";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { useModal } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu, ConfirmationModal } from "@/workbench/ui/components";
import ActionButton from "@/workbench/ui/components/ActionButton";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { useDeleteEnvironmentItem } from "../actions/useDeleteEnvironmentItem";
import { EnvironmentListType } from "../types";

interface EnvironmentItemControlsProps {
  environment: EnvironmentSummary;
  setIsEditing: (isEditing: boolean) => void;
  instruction: Instruction | null;
  type: EnvironmentListType;
}

export const EnvironmentItemControls = ({
  environment,
  setIsEditing,
  instruction,
  type,
}: EnvironmentItemControlsProps) => {
  //const { mutate: activateEnvironment } = useActivateEnvironment();

  const { activePanelId } = useTabbedPaneStore();

  const { showModal: showDeleteModal, setShowModal: setShowDeleteModal, closeModal: closeDeleteModal } = useModal();

  const { handleDeleteEnvironment } = useDeleteEnvironmentItem({ environment, type });

  const [showActionMenu, setShowActionMenu] = useState(false);

  const handleSetActiveEnvironment = (e: MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    /* if (environment.projectId) {
      activateEnvironment({ environmentId: environment.id, projectId: environment.projectId });
    } else {
      activateEnvironment({ environmentId: environment.id });
    } */
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
        depth={type === "GlobalEnvironmentItem" ? 0 : 1}
        dropIndicatorFullWidth
      >
        <Tree.NodeTriggers className="cursor-pointer overflow-hidden">
          <Tree.NodeOrder order={environment.order} />
          <Icon icon={type === "GlobalEnvironmentItem" ? "Environment" : "GroupedEnvironment"} />
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
