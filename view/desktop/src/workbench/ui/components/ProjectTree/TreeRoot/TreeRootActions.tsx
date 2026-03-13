import { useContext } from "react";

import { useModal } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { ActionMenu } from "@/workbench/ui/components";
import { ActionButton } from "@/workbench/ui/components/ActionButton";
import { DeleteProjectModal } from "@/workbench/ui/components/Modals/Project/DeleteProjectModal";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRoot } from "../types";
import { useRefreshProject } from "./dnd/hooks/useRefreshProject";
import { useToggleAllTreeNodes } from "./dnd/hooks/useToggleAllTreeNodes";
import { TreeRootBranchIcon } from "./TreeRootBranchIcon";

interface TreeRootActionsProps {
  node: ProjectTreeRoot;
  setIsRenamingTreeRoot: (isRenaming: boolean) => void;
}

export const TreeRootActions = ({ node, setIsRenamingTreeRoot }: TreeRootActionsProps) => {
  const { allFoldersAreCollapsed, allFoldersAreExpanded, id } = useContext(ProjectTreeContext);

  const { showModal: showDeleteProjectModal, setShowModal: setShowDeleteProjectModal } = useModal();

  const { expandAllNodes, collapseAllNodes } = useToggleAllTreeNodes(id);
  const { refreshProject } = useRefreshProject(id);

  const handleRefresh = () => {
    refreshProject();
  };

  return (
    <>
      <Tree.RootActions>
        {node?.branch && (
          <Tree.ActionLabel className="flex shrink-0 items-center gap-1">
            <div className="flex shrink-0 items-center">
              <span>{node?.branch.behind || 0}</span>
              <TreeRootBranchIcon icon="down" />
            </div>
            <div className="flex shrink-0 items-center">
              <span>{node?.branch.ahead || 0}</span>
              <TreeRootBranchIcon icon="up" />
            </div>
            <div className="text-(--moss-accent) background-(--moss-accent-secondary) rounded-sm px-[5px] text-sm">
              {node?.branch.name}
            </div>
          </Tree.ActionLabel>
        )}

        <Tree.ActionsHover>
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
                <ActionMenu.Item alignWithIcons onClick={() => setIsRenamingTreeRoot(true)}>
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
      </Tree.RootActions>
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
