import { useContext } from "react";

import { ActionMenu } from "@/components";
import { ActionButton } from "@/components/ActionButton";
import { DeleteProjectModal } from "@/components/Modals/Project/DeleteCollectionModal";
import { useModal } from "@/hooks";

import { useRefreshCollection } from "../actions/useRefreshProject";
import { useToggleAllNodes } from "../actions/useToggleAllNodes";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";

interface TreeRootNodeActionsProps {
  node: ProjectTreeRootNode;
  searchInput?: string;
  isRenamingRootNode: boolean;
  setIsAddingRootFileNode: (isAdding: boolean) => void;
  setIsAddingRootFolderNode: (isAdding: boolean) => void;
  setIsRenamingRootNode: (isRenaming: boolean) => void;
}

export const TreeRootNodeActions = ({
  node,
  searchInput,
  isRenamingRootNode,
  setIsAddingRootFileNode,
  setIsAddingRootFolderNode,
  setIsRenamingRootNode,
}: TreeRootNodeActionsProps) => {
  const { displayMode, allFoldersAreCollapsed, allFoldersAreExpanded, id } = useContext(ProjectTreeContext);

  const { showModal: showDeleteCollectionModal, setShowModal: setShowDeleteCollectionModal } = useModal();

  const { expandAllNodes, collapseAllNodes } = useToggleAllNodes(node);
  const { refreshCollection } = useRefreshCollection(id);

  const handleRefresh = () => {
    refreshCollection();
  };

  return (
    <>
      <div className="z-10 flex items-center">
        {node.expanded && !searchInput && !isRenamingRootNode && (
          <div
            className={`hidden items-center opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover/Tree:flex group-hover/Tree:opacity-100`}
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
              onClick={collapseAllNodes}
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
            <ActionMenu.Content className="z-40" align="center">
              <ActionMenu.Item alignWithIcons onClick={() => setIsAddingRootFileNode(true)}>
                Add File
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={() => setIsAddingRootFolderNode(true)}>
                Add Folder
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={() => setIsRenamingRootNode(true)}>
                Rename...
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={handleRefresh}>
                Refresh
              </ActionMenu.Item>
              <ActionMenu.Item alignWithIcons onClick={() => setShowDeleteCollectionModal(true)} icon="Trash">
                Delete
              </ActionMenu.Item>
              <ActionMenu.Item
                alignWithIcons
                disabled={allFoldersAreExpanded}
                onClick={expandAllNodes}
                icon="ExpandAll"
              >
                ExpandAll
              </ActionMenu.Item>
            </ActionMenu.Content>
          </ActionMenu.Portal>
        </ActionMenu.Root>
      </div>
      {showDeleteCollectionModal && (
        <DeleteProjectModal
          id={node.id}
          showModal={showDeleteCollectionModal}
          closeModal={() => setShowDeleteCollectionModal(false)}
        />
      )}
    </>
  );
};
