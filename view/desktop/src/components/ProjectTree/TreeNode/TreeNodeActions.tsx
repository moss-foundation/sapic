import { useContext, useState } from "react";

import { ActionMenu } from "@/components";
import ActionButton from "@/components/ActionButton";
import { Tree } from "@/lib/ui/Tree";

import { useDeleteAndUpdatePeers } from "../actions/useDeleteAndUpdatePeers";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../types";

interface TreeNodeActionsProps {
  node: ProjectTreeNode;
  parentNode: ProjectTreeNode | ProjectTreeRootNode;
  setIsAddingFileNode: (isAdding: boolean) => void;
  setIsAddingFolderNode: (isAdding: boolean) => void;
  setIsRenamingNode: (isRenaming: boolean) => void;
  className?: string;
}

export const TreeNodeActions = ({
  node,
  parentNode,
  setIsAddingFileNode,
  setIsAddingFolderNode,
  setIsRenamingNode,
  className,
}: TreeNodeActionsProps) => {
  const { id } = useContext(ProjectTreeContext);

  const { deleteAndUpdatePeers } = useDeleteAndUpdatePeers(id, node, parentNode);

  const handleDeleteNode = async () => {
    await deleteAndUpdatePeers();
  };

  const [showDropdown, setShowDropdown] = useState(false);

  return (
    <Tree.NodeActions className={className}>
      <Tree.ActionsHover forceVisible={showDropdown}>
        <ActionButton
          customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
          icon="Add"
          onClick={() => setIsAddingFileNode(true)}
        />

        <ActionMenu.Root onOpenChange={setShowDropdown} modal={showDropdown}>
          <ActionMenu.Trigger asChild>
            <ActionButton
              customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
              icon="MoreHorizontal"
            />
          </ActionMenu.Trigger>
          <ActionMenu.Portal>
            <ActionMenu.Content className="z-40" align="center">
              <ActionMenu.Item alignWithIcons onClick={() => setIsAddingFileNode(true)}>
                Add File
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={() => setIsAddingFolderNode(true)}>
                Add Folder
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={() => setIsRenamingNode(true)}>
                Rename...
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={handleDeleteNode} icon="Trash">
                Delete
              </ActionMenu.Item>
            </ActionMenu.Content>
          </ActionMenu.Portal>
        </ActionMenu.Root>
      </Tree.ActionsHover>
    </Tree.NodeActions>
  );
};
