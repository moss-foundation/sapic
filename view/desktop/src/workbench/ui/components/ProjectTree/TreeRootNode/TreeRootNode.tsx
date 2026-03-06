import { useContext, useRef } from "react";

import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { useDraggableRootNode } from "../dnd/hooks/useDraggableRootNode";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTree } from "../types";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootNodeHeaderContent } from "./TreeRootNodeHeaderContent";
import { TreeRootNodeLists } from "./TreeRootNodeLists";
import { TreeRootNodeRenamingForm } from "./TreeRootNodeRenamingForm";
import { calculateShouldRenderRootChildNodes } from "./utils/calculateShouldRenderRootChildNodes";

interface TreeRootNodeProps {
  tree: ProjectTree;
}

export const TreeRootNode = ({ tree }: TreeRootNodeProps) => {
  const { treePaddingLeft, treePaddingRight } = useContext(ProjectTreeContext);

  const draggableHeaderRef = useRef<HTMLLIElement>(null);
  const rootNodeRef = useRef<HTMLUListElement>(null);

  const { data: projects } = useListProjects();
  const { activePanelId } = useTabbedPaneStore();

  const {
    isAddingRootFileNode,
    isAddingRootFolderNode,
    setIsAddingRootFileNode,
    setIsAddingRootFolderNode,
    handleRootAddFormSubmit,
    handleRootAddFormCancel,
  } = useRootNodeAddForm(tree);

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

  const shouldRenderLists = calculateShouldRenderRootChildNodes({ node: tree, isAddingRootFileNode });
  const restrictedNames = projects?.items.map((project) => project.name) ?? [];

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
            shouldRenderChildNodes={shouldRenderLists}
            restrictedNames={restrictedNames}
            handleRenamingFormSubmit={handleRenamingRootNodeFormSubmit}
            handleRenamingFormCancel={handleRenamingRootNodeFormCancel}
          />
        ) : (
          <TreeRootNodeHeaderContent
            node={tree}
            isAddingRootFileNode={isAddingRootFileNode}
            setIsAddingRootFileNode={setIsAddingRootFileNode}
            setIsAddingRootFolderNode={setIsAddingRootFolderNode}
            setIsRenamingRootNode={setIsRenamingRootNode}
          />
        )}
      </Tree.RootNodeHeader>

      {shouldRenderLists && (
        <TreeRootNodeLists
          tree={tree}
          isAddingRootFileNode={isAddingRootFileNode}
          isAddingRootFolderNode={isAddingRootFolderNode}
          handleRootAddFormSubmit={handleRootAddFormSubmit}
          handleRootAddFormCancel={handleRootAddFormCancel}
        />
      )}
    </Tree.RootNode>
  );
};
