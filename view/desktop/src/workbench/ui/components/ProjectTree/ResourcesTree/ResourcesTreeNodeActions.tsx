import { useContext, useState } from "react";

import { Tree } from "@/lib/ui/Tree";
import { ActionMenu } from "@/workbench/ui/components";
import ActionButton from "@/workbench/ui/components/ActionButton";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ResourceNode } from "../types";
import { useDeleteAndUpdateResourceNodePeers } from "./hooks/useDeleteAndUpdateResourceNodePeers";

interface ResourcesTreeNodeActionsProps {
  node: ResourceNode;
  parentNode?: ResourceNode;
  setIsAddingFileNode: (isAdding: boolean) => void;
  setIsAddingFolderNode: (isAdding: boolean) => void;
  setIsRenamingNode: (isRenaming: boolean) => void;
  className?: string;
}

export const ResourcesTreeNodeActions = ({
  node,
  parentNode,
  setIsAddingFileNode,
  setIsAddingFolderNode,
  setIsRenamingNode,
  className,
}: ResourcesTreeNodeActionsProps) => {
  const { id } = useContext(ProjectTreeContext);

  const [showDropdown, setShowDropdown] = useState(false);

  const { deleteAndUpdatePeers } = useDeleteAndUpdateResourceNodePeers({ projectId: id, node, parentNode });

  const handleDeleteNode = async () => {
    await deleteAndUpdatePeers();
  };

  return (
    <Tree.NodeActions className={className}>
      <Tree.ActionsHover forceVisible={showDropdown}>
        <ActionButton hoverVariant="list" icon="Add" onClick={() => setIsAddingFileNode(true)} />

        <ActionMenu.Root onOpenChange={setShowDropdown} modal={showDropdown}>
          <ActionMenu.Trigger asChild>
            <ActionButton hoverVariant="list" icon="MoreHorizontal" />
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
