import { useContext, useRef } from "react";

import { useStreamedCollections } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

import { ActiveNodeIndicator } from "../ActiveNodeIndicator";
import { DropIndicatorForDir } from "../DropIndicatorForDir";
import { DropIndicatorForTrigger } from "../DropIndicatorForTrigger";
import { TreeContext } from "../Tree";
import { TreeRootNodeProps } from "../types";
import { calculateShouldRenderRootChildNodes } from "../utils";
import { useDraggableRootNode } from "./hooks/useDraggableRootNode";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootNodeActions } from "./TreeRootNodeActions";
import { TreeRootNodeButton } from "./TreeRootNodeButton";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootNodeRenameForm } from "./TreeRootNodeRenameForm";

export const TreeRootNode = ({ node }: TreeRootNodeProps) => {
  const { searchInput, treePaddingLeft, treePaddingRight } = useContext(TreeContext);

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetRootRef = useRef<HTMLDivElement>(null);

  const { data: streamedCollections } = useStreamedCollections();
  const { activePanelId } = useTabbedPaneStore();

  const {
    isAddingRootNodeFile,
    isAddingRootNodeFolder,
    setIsAddingRootNodeFile,
    setIsAddingRootNodeFolder,
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
    triggerRef: draggableRootRef,
    node,
    isRenamingNode: isRenamingRootNode,
  });

  const shouldRenderRootChildNodes = calculateShouldRenderRootChildNodes(
    node,
    isDragging,
    isAddingRootNodeFile,
    isRenamingRootNode
  );

  const restrictedNames = streamedCollections?.map((collection) => collection.name) ?? [];

  return (
    <div ref={dropTargetRootRef} className={cn("group relative w-full", {})}>
      <DropIndicatorForDir isChildDropBlocked={isChildDropBlocked} instruction={instruction} />
      <DropIndicatorForTrigger instruction={instruction} />

      <div
        ref={draggableRootRef}
        className={cn("group/TreeNode relative flex w-full min-w-0 items-center justify-between py-0.75")}
        style={{
          paddingLeft: treePaddingLeft,
          paddingRight: treePaddingRight,
        }}
      >
        <ActiveNodeIndicator isActive={activePanelId === node.id} />

        {isRenamingRootNode ? (
          <TreeRootNodeRenameForm
            node={node}
            shouldRenderChildNodes={shouldRenderRootChildNodes}
            restrictedNames={restrictedNames}
            handleRenamingFormSubmit={handleRenamingRootNodeFormSubmit}
            handleRenamingFormCancel={handleRenamingRootNodeFormCancel}
          />
        ) : (
          <TreeRootNodeButton
            node={node}
            searchInput={searchInput}
            shouldRenderChildNodes={shouldRenderRootChildNodes}
          />
        )}

        <TreeRootNodeActions
          node={node}
          searchInput={searchInput}
          isRenamingRootNode={isRenamingRootNode}
          setIsAddingRootFileNode={setIsAddingRootNodeFile}
          setIsAddingRootFolderNode={setIsAddingRootNodeFolder}
          setIsRenamingRootNode={setIsRenamingRootNode}
        />
      </div>

      {shouldRenderRootChildNodes && (
        <TreeRootNodeChildren
          node={node}
          isAddingRootFileNode={isAddingRootNodeFile}
          isAddingRootFolderNode={isAddingRootNodeFolder}
          handleAddFormRootSubmit={handleRootAddFormSubmit}
          handleAddFormRootCancel={handleRootAddFormCancel}
        />
      )}
    </div>
  );
};
