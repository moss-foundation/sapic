import { useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRoot } from "../types";
import { TreeRootActions } from "./TreeRootActions";

interface TreeRootHeaderContentProps {
  node: ProjectTreeRoot;
  setIsRenamingTreeRoot: (isRenaming: boolean) => void;
}

export const TreeRootHeaderContent = ({ node, setIsRenamingTreeRoot }: TreeRootHeaderContentProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { id, showOrders, showTreeRootIds } = useContext(ProjectTreeContext);

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
      <Tree.RootDetails>
        <Tree.RootTriggers className="overflow-hidden">
          <Tree.RootIcon handleIconClick={handleIconClick} isFolderExpanded={node.expanded} />
          {showOrders && <Tree.RootOrder order={node.order} />}
          <Tree.RootLabel label={node.name} onClick={handleLabelClick} />
          {showTreeRootIds && <Tree.RootLabel label={node.id} />}
        </Tree.RootTriggers>

        <TreeRootActions node={node} setIsRenamingTreeRoot={setIsRenamingTreeRoot} />
      </Tree.RootDetails>
    </>
  );
};
