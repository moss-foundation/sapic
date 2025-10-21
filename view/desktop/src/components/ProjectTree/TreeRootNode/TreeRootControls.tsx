import { useContext } from "react";

import { ActionMenu } from "@/components";
import ActionButton from "@/components/ActionButton";
import { DeleteProjectModal } from "@/components/Modals/Project/DeleteProjectModal";
import { useModal, useUpdateProject } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { useRefreshProject } from "../actions/useRefreshProject";
import { useToggleAllNodes } from "../actions/useToggleAllNodes";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";

interface TreeRootControlsProps {
  node: ProjectTreeRootNode;
  setIsAddingRootFileNode: (isAdding: boolean) => void;
  setIsAddingRootFolderNode: (isAdding: boolean) => void;
  setIsRenamingRootNode: (isRenaming: boolean) => void;
}

export const TreeRootControls = ({
  node,
  setIsAddingRootFileNode,
  setIsAddingRootFolderNode,
  setIsRenamingRootNode,
}: TreeRootControlsProps) => {
  const { allFoldersAreExpanded, allFoldersAreCollapsed, id, showOrders } = useContext(ProjectTreeContext);

  const { addOrFocusPanel } = useTabbedPaneStore();

  const { mutateAsync: updateProject } = useUpdateProject();
  const { expandAllNodes, collapseAllNodes } = useToggleAllNodes(node);
  const { refreshProject } = useRefreshProject(id);

  const { showModal: showDeleteProjectModal, setShowModal: setShowDeleteProjectModal } = useModal();

  const handleRefresh = () => {
    refreshProject();
  };

  const handleIconClick = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    await updateProject({
      id,
      expanded: !node.expanded,
    });
  };

  const handleLabelClick = async () => {
    if (!node.expanded) {
      await updateProject({
        id,
        expanded: true,
      });
    }

    addOrFocusPanel({
      id,
      title: node.name,
      component: "ProjectSettings",
      params: {
        projectId: id,
        iconType: "Project",
      },
    });
  };

  return (
    <>
      <Tree.RootNodeControls>
        <Tree.RootNodeTriggers className="overflow-hidden">
          <Tree.RootNodeIcon handleIconClick={handleIconClick} isFolderExpanded={node.expanded} />
          {showOrders && <Tree.RootNodeOrder order={node.order} />}
          <Tree.RootNodeLabel label={node.name} onClick={handleLabelClick} />
        </Tree.RootNodeTriggers>

        <Tree.RootNodeActions>
          {node?.branch && (
            <Tree.ActionLabel className="flex shrink-0 items-center gap-1">
              {node?.branch && <div>{node?.branch.name}</div>}
              {!!(node?.branch?.ahead && node?.branch?.ahead > 0) && (
                <div className="flex shrink-0 gap-0.5 text-green-500">{node?.branch.ahead} ↑</div>
              )}
              {!!(node?.branch?.behind && node?.branch?.behind > 0) && (
                <div className="flex shrink-0 gap-0.5 text-red-500">{node?.branch.behind} ↓</div>
              )}
            </Tree.ActionLabel>
          )}

          <Tree.ActionsHover>
            <ActionButton icon="Add" onClick={() => setIsAddingRootFileNode(true)} hoverVariant="list" />
            <ActionButton
              icon="CollapseAll"
              disabled={allFoldersAreCollapsed}
              onClick={collapseAllNodes}
              hoverVariant="list"
            />
          </Tree.ActionsHover>
          <Tree.ActionsPersistent>
            <ActionMenu.Root>
              <ActionMenu.Trigger asChild>
                <ActionButton icon="MoreHorizontal" hoverVariant="list" />
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
          </Tree.ActionsPersistent>
        </Tree.RootNodeActions>
      </Tree.RootNodeControls>

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
