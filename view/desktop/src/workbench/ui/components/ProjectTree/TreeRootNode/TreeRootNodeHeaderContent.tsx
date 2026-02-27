import { useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";
import { TreeRootNodeActions } from "./TreeRootNodeActions";

interface TreeRootNodeHeaderContentProps {
  node: ProjectTreeRootNode;
  isAddingRootFileNode: boolean;
  setIsAddingRootFileNode: (isAdding: boolean) => void;
  setIsAddingRootFolderNode: (isAdding: boolean) => void;
  setIsRenamingRootNode: (isRenaming: boolean) => void;
}

export const TreeRootNodeHeaderContent = ({
  node,
  isAddingRootFileNode,
  setIsAddingRootFileNode,
  setIsAddingRootFolderNode,
  setIsRenamingRootNode,
}: TreeRootNodeHeaderContentProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { id, showOrders, showRootNodeIds } = useContext(ProjectTreeContext);

  const rotateIcon = node.expanded || isAddingRootFileNode;

  const { addOrFocusPanel } = useTabbedPaneStore();

  const handleIconClick = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    await treeItemStateService.putExpanded(node.id, !node.expanded, currentWorkspaceId);
  };

  const handleLabelClick = async () => {
    if (!node.expanded) {
      await treeItemStateService.putExpanded(node.id, true, currentWorkspaceId);
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
      <Tree.RootNodeDetails>
        <Tree.RootNodeTriggers className="overflow-hidden">
          <Tree.RootNodeIcon handleIconClick={handleIconClick} isFolderExpanded={rotateIcon} />
          {showOrders && <Tree.RootNodeOrder order={node.order} />}
          <Tree.RootNodeLabel label={node.name} onClick={handleLabelClick} />
          {showRootNodeIds && <Tree.RootNodeLabel label={node.id} />}
        </Tree.RootNodeTriggers>

        <TreeRootNodeActions
          node={node}
          setIsAddingRootFileNode={setIsAddingRootFileNode}
          setIsAddingRootFolderNode={setIsAddingRootFolderNode}
          setIsRenamingRootNode={setIsRenamingRootNode}
        />
      </Tree.RootNodeDetails>
    </>
  );
};
