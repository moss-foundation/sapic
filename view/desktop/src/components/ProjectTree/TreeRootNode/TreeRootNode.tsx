import { useRef } from "react";

import { useStreamProjects } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { ProjectTreeRootNodeProps } from "../types";
import { calculateShouldRenderRootChildNodes } from "../utils";
import { useDraggableRootNode } from "./hooks/useDraggableRootNode";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootControls } from "./TreeRootControls";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootNodeRenamingForm } from "./TreeRootNodeRenamingForm";

export const TreeRootNode = ({ node }: ProjectTreeRootNodeProps) => {
  const draggableHeaderRef = useRef<HTMLLIElement>(null);
  const dropTargetRootRef = useRef<HTMLUListElement>(null);

  const { data: streamedProjects } = useStreamProjects();
  const { activePanelId } = useTabbedPaneStore();

  const {
    isAddingRootFileNode,
    isAddingRootFolderNode,
    setIsAddingRootFileNode,
    setIsAddingRootFolderNode,
    handleRootAddFormCancel,
    handleRootAddFormSubmit,
  } = useRootNodeAddForm(node);

  const {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  } = useRootNodeRenamingForm(node);

  const { isDragging, instruction, dirInstruction } = useDraggableRootNode({
    dirRef: dropTargetRootRef,
    triggerRef: draggableHeaderRef,
    node,
    isRenamingNode: isRenamingRootNode,
  });

  const shouldRenderRootChildNodes = calculateShouldRenderRootChildNodes(
    node,
    isAddingRootFileNode,
    isRenamingRootNode
  );

  const restrictedNames = streamedProjects?.map((project) => project.name) ?? [];

  return (
    <Tree.RootNode
      ref={dropTargetRootRef}
      instruction={instruction}
      combineInstruction={dirInstruction}
      isDragging={isDragging}
    >
      <Tree.RootNodeHeader ref={draggableHeaderRef} isActive={activePanelId === node.id}>
        {isRenamingRootNode ? (
          <TreeRootNodeRenamingForm
            node={node}
            shouldRenderChildNodes={shouldRenderRootChildNodes}
            restrictedNames={restrictedNames}
            handleRenamingFormSubmit={handleRenamingRootNodeFormSubmit}
            handleRenamingFormCancel={handleRenamingRootNodeFormCancel}
          />
        ) : (
          <TreeRootControls
            node={node}
            setIsAddingRootFileNode={setIsAddingRootFileNode}
            setIsAddingRootFolderNode={setIsAddingRootFolderNode}
            setIsRenamingRootNode={setIsRenamingRootNode}
          />
        )}
      </Tree.RootNodeHeader>

      {shouldRenderRootChildNodes && (
        <TreeRootNodeChildren
          node={node}
          isAddingRootFileNode={isAddingRootFileNode}
          isAddingRootFolderNode={isAddingRootFolderNode}
          handleAddFormRootSubmit={handleRootAddFormSubmit}
          handleAddFormRootCancel={handleRootAddFormCancel}
        />
      )}
    </Tree.RootNode>
  );
};
