import { useContext, useState } from "react";

import { ActionMenu, TreeContext } from "@/components";
import ActionButton from "@/components/ActionButton";
import { cn } from "@/utils";

import { useDeleteAndUpdatePeers } from "../actions/useDeleteAndUpdatePeers";
import { TreeCollectionNode } from "../types";

interface TreeNodeActionsProps {
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
  setIsAddingFileNode: (isAdding: boolean) => void;
  setIsAddingFolderNode: (isAdding: boolean) => void;
  setIsRenamingNode: (isRenaming: boolean) => void;
}

export const TreeNodeActions = ({
  node,
  parentNode,
  setIsAddingFileNode,
  setIsAddingFolderNode,
  setIsRenamingNode,
}: TreeNodeActionsProps) => {
  const { id } = useContext(TreeContext);
  const { deleteAndUpdatePeers } = useDeleteAndUpdatePeers(id, node, parentNode);

  const handleDeleteNode = async () => {
    await deleteAndUpdatePeers();
  };

  const [showDropdown, setShowDropdown] = useState(false);

  return (
    <div
      className={cn(
        "hidden items-center gap-0.5 opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover/treeNode:flex group-hover/treeNode:opacity-100",
        {
          "flex opacity-100": showDropdown,
        }
      )}
    >
      <ActionButton
        size="small"
        customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
        icon="Plus"
        onClick={() => setIsAddingFolderNode(true)}
      />

      <div className="z-10 flex items-center">
        <ActionMenu.Root onOpenChange={setShowDropdown} modal={showDropdown}>
          <ActionMenu.Trigger asChild className="">
            <ActionButton
              size="small"
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
      </div>
    </div>
  );
};
