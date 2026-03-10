import { useContext, useRef } from "react";

import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { useDraggableRootNode } from "../dnd/hooks/useDraggableRootNode";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTree } from "../types";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootNodeHeaderContent } from "./TreeRootNodeHeaderContent";
import { TreeRootNodeLists } from "./TreeRootNodeLists";
import { TreeRootNodeRenamingForm } from "./TreeRootNodeRenamingForm";

interface TreeRootNodeProps {
  tree: ProjectTree;
}

export const TreeRootNode = ({ tree }: TreeRootNodeProps) => {
  const { treePaddingLeft, treePaddingRight } = useContext(ProjectTreeContext);

  const draggableHeaderRef = useRef<HTMLLIElement>(null);
  const rootNodeRef = useRef<HTMLUListElement>(null);

  const { data: projectSummaries } = useGetAllLocalProjectSummaries();
  const { activePanelId } = useTabbedPaneStore();

  const {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  } = useRootNodeRenamingForm(tree);

  const { isDragging, instruction } = useDraggableRootNode({
    nodeRef: rootNodeRef,
    headerRef: draggableHeaderRef,
    node: tree,
    isRenamingNode: isRenamingRootNode,
  });

  const restrictedNames = projectSummaries?.map((projectSummary) => projectSummary.name) ?? [];

  return (
    <Tree.RootNode ref={rootNodeRef} reorderInstruction={instruction} isDragging={isDragging}>
      <Tree.RootNodeHeader
        ref={draggableHeaderRef}
        isActive={activePanelId === tree.id}
        treePaddingLeft={treePaddingLeft}
        treePaddingRight={treePaddingRight}
      >
        {isRenamingRootNode ? (
          <TreeRootNodeRenamingForm
            node={tree}
            shouldRenderChildNodes={tree.expanded}
            restrictedNames={restrictedNames}
            handleRenamingFormSubmit={handleRenamingRootNodeFormSubmit}
            handleRenamingFormCancel={handleRenamingRootNodeFormCancel}
          />
        ) : (
          <TreeRootNodeHeaderContent node={tree} setIsRenamingRootNode={setIsRenamingRootNode} />
        )}
      </Tree.RootNodeHeader>

      {tree.expanded && <TreeRootNodeLists tree={tree} />}
    </Tree.RootNode>
  );
};
