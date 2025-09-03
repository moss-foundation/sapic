import { useRef } from "react";

import { useStreamCollections } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { TreeRootNodeProps } from "../types";
import { calculateShouldRenderRootChildNodes } from "../utils";
import { useDraggableRootNode } from "./hooks/useDraggableRootNode";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootControls } from "./TreeRootControls";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootRenamingForm } from "./TreeRootRenamingForm";

export const TreeRootNode = ({ node }: TreeRootNodeProps) => {
  const draggableHeaderRef = useRef<HTMLLIElement>(null);
  const dropTargetRootRef = useRef<HTMLUListElement>(null);

  const { data: streamedCollections } = useStreamCollections();
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

  const { isDragging, isChildDropBlocked, instruction } = useDraggableRootNode({
    dirRef: dropTargetRootRef,
    triggerRef: draggableHeaderRef,
    node,
    isRenamingNode: isRenamingRootNode,
  });

  const shouldRenderRootChildNodes = calculateShouldRenderRootChildNodes(
    node,
    isDragging,
    isAddingRootFileNode,
    isRenamingRootNode
  );

  const restrictedNames = streamedCollections?.map((collection) => collection.name) ?? [];

  return (
    <Tree.RootNode ref={dropTargetRootRef} isChildDropBlocked={isChildDropBlocked} instruction={instruction}>
      <Tree.RootNodeHeader ref={draggableHeaderRef} isActive={activePanelId === node.id}>
        {isRenamingRootNode ? (
          <TreeRootRenamingForm
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
