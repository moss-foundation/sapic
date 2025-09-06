import { MouseEvent, useState } from "react";

import { ActionMenu } from "@/components";
import ActionButton from "@/components/ActionButton";
import { useModal } from "@/hooks";
import { useActivateEnvironment } from "@/hooks/workspace/environment/useActivateEnvironment";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { EnvironmentListType } from "./types";

interface EnvironmentItemControlsProps {
  environment: StreamEnvironmentsEvent;
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
  const { mutate: activateEnvironment } = useActivateEnvironment();

  const { activePanelId } = useTabbedPaneStore();

  const { showModal: showDeleteModal, setShowModal: setShowDeleteModal, closeModal: setHideDeleteModal } = useModal();

  const [showActionMenu, setShowActionMenu] = useState(false);

  const handleSetActiveEnvironment = (e: MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    activateEnvironment({ environmentId: environment.id });
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
          <Icon icon={type === "GlobalEnvironmentItem" ? "Environment" : "GroupedEnvironment"} />
          <span className="truncate">{environment.name}</span>
          <span className="text-(--moss-secondary-text)">({environment.totalVariables})</span>
        </Tree.NodeTriggers>

        <Tree.NodeActions>
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

      {/* {showDeleteModal && (
        <ConfirmationModal
          showModal={showDeleteModal}
          closeModal={setHideDeleteModal}
          title="Delete Environment"
          message={`Are you sure you want to delete ${environment.name} environment?`}
          onConfirm={handleDeleteEnvironment}
        />
      )} */}
    </>
  );
};
