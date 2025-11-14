import { useContext } from "react";

import { useModal } from "@/hooks";
import { ActionMenu } from "@/workbench/ui/components";
import { ActionButton } from "@/workbench/ui/components/ActionButton";
import { DeleteProjectModal } from "@/workbench/ui/components/Modals/Project/DeleteProjectModal";

import { useRefreshProject } from "../actions/useRefreshProject";
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

  const { showModal: showDeleteProjectModal, setShowModal: setShowDeleteProjectModal } = useModal();

  const { expandAllNodes, collapseAllNodes } = useToggleAllNodes(node);
  const { refreshProject } = useRefreshProject(id);

  const handleRefresh = () => {
    refreshProject();
  };

  return (
    <>
      <div className="z-10 flex items-center">
        {node.expanded && !searchInput && !isRenamingRootNode && (
          <div
            className={`transition-discrete hidden items-center opacity-0 transition-[display,opacity] duration-100 group-hover/Tree:flex group-hover/Tree:opacity-100`}
          >
            {displayMode === "LIVE" && (
              <ActionButton hoverVariant="list" icon="Add" onClick={() => setIsAddingRootFileNode(true)} />
            )}
            <ActionButton
              icon="CollapseAll"
              disabled={allFoldersAreCollapsed}
              onClick={collapseAllNodes}
              hoverVariant="list"
            />
          </div>
        )}
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <ActionButton hoverVariant="list" icon="MoreHorizontal" />
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
              <ActionMenu.Item alignWithIcons onClick={() => setShowDeleteProjectModal(true)} icon="Trash">
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
      {showDeleteProjectModal && (
        <DeleteProjectModal
          id={node.id}
          showModal={showDeleteProjectModal}
          closeModal={() => setShowDeleteProjectModal(false)}
        />
      )}
    </>
  );
};
