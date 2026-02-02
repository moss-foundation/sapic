import { useContext } from "react";

import { useCurrentWorkspace, useModal } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { usePutTreeItemState } from "@/workbench/adapters/tanstackQuery/treeItemState/useUpdateTreeItemState";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu } from "@/workbench/ui/components";
import ActionButton from "@/workbench/ui/components/ActionButton";
import { DeleteProjectModal } from "@/workbench/ui/components/Modals/Project/DeleteProjectModal";

import { useRefreshProject } from "../actions/useRefreshProject";
import { useToggleAllNodes } from "../actions/useToggleAllNodes";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";
import { TreeRootNodeBranchIcon } from "./TreeRootNodeBranchIcon";

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
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { allFoldersAreExpanded, allFoldersAreCollapsed, id, showOrders, showRootNodeIds } =
    useContext(ProjectTreeContext);

  const { addOrFocusPanel } = useTabbedPaneStore();

  const { expandAllNodes, collapseAllNodes } = useToggleAllNodes(node);
  const { refreshProject } = useRefreshProject(id);
  const { mutateAsync: updateTreeItemState } = usePutTreeItemState();
  const { showModal: showDeleteProjectModal, setShowModal: setShowDeleteProjectModal } = useModal();

  const handleRefresh = () => {
    refreshProject();
  };

  const handleIconClick = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    await updateTreeItemState({
      treeItemState: { id: node.id, order: node.order ?? 0, expanded: !node.expanded },
      workspaceId: currentWorkspaceId,
    });
  };

  const handleLabelClick = async () => {
    if (!node.expanded) {
      await updateTreeItemState({
        treeItemState: { id: node.id, order: node.order ?? 0, expanded: true },
        workspaceId: currentWorkspaceId,
      });
    }

    addOrFocusPanel({
      id,
      title: node.name,
      component: "ProjectSettingsView",
      params: {
        projectId: id,
        tabIcon: "Project",
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
          {showRootNodeIds && <Tree.RootNodeLabel label={node.id} />}
        </Tree.RootNodeTriggers>

        <Tree.RootNodeActions>
          {node?.branch && (
            <Tree.ActionLabel className="flex shrink-0 items-center gap-1">
              <div className="flex shrink-0 items-center">
                <span>{node?.branch.behind || 0}</span>
                <TreeRootNodeBranchIcon icon="down" />
              </div>
              <div className="flex shrink-0 items-center">
                <span>{node?.branch.ahead || 0}</span>
                <TreeRootNodeBranchIcon icon="up" />
              </div>
              <div className="text-(--moss-accent) background-(--moss-accent-secondary) rounded-sm px-[5px] text-sm">
                {node?.branch.name}
              </div>
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
