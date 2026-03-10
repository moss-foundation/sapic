import { useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTree } from "../types";
import { TreeRootNodeActions } from "./TreeRootNodeActions";

interface TreeRootNodeHeaderContentProps {
  node: ProjectTree;
  setIsRenamingRootNode: (isRenaming: boolean) => void;
}

export const TreeRootNodeHeaderContent = ({ node, setIsRenamingRootNode }: TreeRootNodeHeaderContentProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { id, showOrders, showRootNodeIds } = useContext(ProjectTreeContext);

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
          <Tree.RootNodeIcon handleIconClick={handleIconClick} isFolderExpanded={node.expanded} />
          {showOrders && <Tree.RootNodeOrder order={node.order} />}
          <Tree.RootNodeLabel label={node.name} onClick={handleLabelClick} />
          {showRootNodeIds && <Tree.RootNodeLabel label={node.id} />}
        </Tree.RootNodeTriggers>

        <TreeRootNodeActions node={node} setIsRenamingRootNode={setIsRenamingRootNode} />
      </Tree.RootNodeDetails>
    </>
  );
};
