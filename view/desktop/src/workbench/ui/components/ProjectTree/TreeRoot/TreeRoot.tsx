import { useRef } from "react";

import { useGetAllLocalProjectSummaries } from "@/db/projectSummaries/hooks/useGetAllLocalProjectSummaries";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { NODE_OFFSET, TREE_HEADER_PADDING_RIGHT } from "../constants";
import { ProjectTreeRoot } from "../types";
import { useDraggableTreeRoot } from "./dnd/hooks/useDraggableTreeRoot";
import { TreeRootRenamingForm } from "./forms/TreeRootRenamingForm";
import { useTreeRootRenamingForm } from "./hooks/useTreeRootRenamingForm";
import { TreeRootHeaderContent } from "./TreeRootHeaderContent";
import { TreeRootLists } from "./TreeRootLists";

interface TreeRootProps {
  tree: ProjectTreeRoot;
}

export const TreeRoot = ({ tree }: TreeRootProps) => {
  const draggableHeaderRef = useRef<HTMLLIElement>(null);
  const treeRootRef = useRef<HTMLUListElement>(null);

  const { data: projectSummaries } = useGetAllLocalProjectSummaries();
  const { activePanelId } = useTabbedPaneStore();

  const {
    isRenamingTreeRoot,
    setIsRenamingTreeRoot,
    handleRenamingTreeRootFormSubmit,
    handleRenamingTreeRootFormCancel,
  } = useTreeRootRenamingForm(tree);

  const { isDragging, instruction } = useDraggableTreeRoot({
    nodeRef: treeRootRef,
    headerRef: draggableHeaderRef,
    node: tree,
    isRenamingTreeRoot,
  });

  const restrictedNames = projectSummaries?.map((projectSummary) => projectSummary.name) ?? [];

  return (
    <Tree.Root ref={treeRootRef} reorderInstruction={instruction} isDragging={isDragging}>
      <Tree.RootHeader
        ref={draggableHeaderRef}
        isActive={activePanelId === tree.id}
        paddingLeft={NODE_OFFSET * 2}
        paddingRight={TREE_HEADER_PADDING_RIGHT}
      >
        {isRenamingTreeRoot ? (
          <TreeRootRenamingForm
            node={tree}
            shouldRenderChildNodes={tree.expanded}
            restrictedNames={restrictedNames}
            handleRenamingFormSubmit={handleRenamingTreeRootFormSubmit}
            handleRenamingFormCancel={handleRenamingTreeRootFormCancel}
          />
        ) : (
          <TreeRootHeaderContent node={tree} setIsRenamingTreeRoot={setIsRenamingTreeRoot} />
        )}
      </Tree.RootHeader>

      {tree.expanded && <TreeRootLists tree={tree} />}
    </Tree.Root>
  );
};
