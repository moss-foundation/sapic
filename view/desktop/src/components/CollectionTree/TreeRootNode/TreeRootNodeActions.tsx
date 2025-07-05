import { useContext, useState } from "react";

import { ActionMenu, TreeContext } from "@/components";
import { ActionButton } from "@/components/ActionButton";
import { DeleteCollectionModal } from "@/components/Modals/Collection/DeleteCollectionModal";

import { TreeCollectionRootNode } from "../types";

interface TreeRootNodeActionsProps {
  node: TreeCollectionRootNode;
  searchInput?: string;
  isRenamingRootNode: boolean;
  setIsAddingRootFileNode: (isAdding: boolean) => void;
  setIsAddingRootFolderNode: (isAdding: boolean) => void;
  setIsRenamingRootNode: (isRenaming: boolean) => void;
  allFoldersAreCollapsed: boolean;
  allFoldersAreExpanded: boolean;
  handleCollapseAll: () => void;
  handleExpandAll: () => void;
}

export const TreeRootNodeActions = ({
  node,
  searchInput,
  isRenamingRootNode,
  setIsAddingRootFileNode,
  setIsAddingRootFolderNode,
  setIsRenamingRootNode,
  allFoldersAreCollapsed,
  allFoldersAreExpanded,
  handleCollapseAll,
  handleExpandAll,
}: TreeRootNodeActionsProps) => {
  const { displayMode } = useContext(TreeContext);
  const [showDeleteCollectionModal, setShowDeleteCollectionModal] = useState(false);

  return (
    <>
      <div className="z-10 flex items-center">
        {node.expanded && !searchInput && !isRenamingRootNode && (
          <div
            className={`hidden items-center opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover:flex group-hover:opacity-100`}
          >
            {displayMode === "REQUEST_FIRST" && (
              <ActionButton
                customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
                icon="Add"
                onClick={() => setIsAddingRootFileNode(true)}
              />
            )}
            <ActionButton
              customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
              icon="CollapseAll"
              disabled={allFoldersAreCollapsed}
              onClick={handleCollapseAll}
            />
          </div>
        )}
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <ActionButton
              customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
              icon="MoreHorizontal"
            />
          </ActionMenu.Trigger>
          <ActionMenu.Portal>
            <ActionMenu.Content className="z-30" align="center">
              <ActionMenu.Item alignWithIcons onClick={() => setIsAddingRootFileNode(true)}>
                Add File
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={() => setIsAddingRootFolderNode(true)}>
                Add Folder
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={() => setIsRenamingRootNode(true)}>
                Rename...
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons>Refresh</ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={() => setShowDeleteCollectionModal(true)} icon="Trash">
                Delete
              </ActionMenu.Item>
              <ActionMenu.Item
                alignWithIcons
                disabled={allFoldersAreExpanded}
                onClick={handleExpandAll}
                icon="ExpandAll"
              >
                ExpandAll
              </ActionMenu.Item>
            </ActionMenu.Content>
          </ActionMenu.Portal>
        </ActionMenu.Root>
      </div>
      {showDeleteCollectionModal && (
        <DeleteCollectionModal
          id={node.id}
          showModal={showDeleteCollectionModal}
          closeModal={() => setShowDeleteCollectionModal(false)}
        />
      )}
    </>
  );
};
