import { useContext, useRef } from "react";

import { DropIndicator } from "@/components/DropIndicator";
import { cn } from "@/utils";

import { useDraggableRootNode } from "../hooks/useDraggableRootNode";
import { useDropTargetRootNode } from "../hooks/useDropTargetRootNode";
import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootNodeActions } from "./TreeRootNodeActions";
import { TreeRootNodeButton } from "./TreeRootNodeButton";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootNodeRenameForm } from "./TreeRootNodeRenameForm";

const shouldRenderChildNodesFn = (node: TreeCollectionRootNode, isDragging: boolean) => {
  if (!node.expanded) {
    return false;
  }

  if (isDragging) {
    return false;
  }

  return true;
};

export interface TreeRootNodePropsV2 {
  onNodeUpdate: (node: TreeCollectionRootNode) => void;
  onRootUpdate: (node: TreeCollectionRootNode) => void;
  node: TreeCollectionRootNode;
}

export const TreeRootNode = ({ node, onNodeUpdate, onRootUpdate }: TreeRootNodePropsV2) => {
  const { treeId, allFoldersAreCollapsed, allFoldersAreExpanded, searchInput, rootOffset } = useContext(TreeContext);

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetRootRef = useRef<HTMLDivElement>(null);

  //   const handleExpandAll = () => {
  //     const newNode = expandAllNodes(node);
  //     onNodeUpdate({
  //       ...node,
  //       childNodes: newNode.childNodes,
  //     });
  //   };

  //   const handleCollapseAll = () => {
  //     const newNode = collapseAllNodes(node);
  //     onNodeUpdate({
  //       ...node,
  //       childNodes: newNode.childNodes,
  //     });
  //   };

  //   const {
  //     isAddingFileNode: isAddingRootFileNode,
  //     isAddingFolderNode: isAddingRootFolderNode,
  //     setIsAddingFileNode: setIsAddingRootFileNode,
  //     setIsAddingFolderNode: setIsAddingRootFolderNode,
  //     handleAddFormSubmit: handleAddFormRootSubmit,
  //     handleAddFormCancel: handleAddFormRootCancel,
  //   } = useNodeAddForm(node, onNodeUpdate);

  const {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  } = useRootNodeRenamingForm(node, onRootUpdate);

  const { closestEdge, isDragging } = useDraggableRootNode(draggableRootRef, node, treeId, isRenamingRootNode);
  useDropTargetRootNode(node, treeId, dropTargetRootRef);

  //   useEffect(() => {
  //     const handleNewCollectionWasCreated = (event: Event) => {
  //       const customEvent = event as CustomEvent<{ treeId: string }>;
  //       if (treeId === customEvent.detail.treeId) {
  //         setIsRenamingRootNode(true);
  //       }
  //     };
  //     window.addEventListener("newCollectionWasCreated", handleNewCollectionWasCreated);
  //     return () => {
  //       window.removeEventListener("newCollectionWasCreated", handleNewCollectionWasCreated as EventListener);
  //     };
  //   }, [setIsRenamingRootNode, treeId]);
  //

  const shouldRenderChildNodes = shouldRenderChildNodesFn(node, isDragging);
  // !!searchInput ||
  // (!searchInput && node.isFolder && node.isExpanded) ||
  // isAddingRootFileNode ||
  // isAddingRootFolderNode;

  return (
    <div
      ref={dropTargetRootRef}
      className={cn("group relative w-full", {
        "hidden": isDragging,
      })}
    >
      <div
        ref={draggableRootRef}
        className="group/TreeRootHeader relative flex w-full min-w-0 items-center justify-between gap-1 py-[5px] pr-2"
        style={{ paddingLeft: rootOffset, paddingRight: rootOffset }}
      >
        <span
          className={cn(
            "group-hover/TreeRootHeader:background-(--moss-secondary-background-hover) absolute inset-x-1 h-[calc(100%-8px)] w-[calc(100%-8px)] rounded-sm"
          )}
        />

        {isRenamingRootNode ? (
          <TreeRootNodeRenameForm
            node={node}
            handleRenamingFormSubmit={handleRenamingRootNodeFormSubmit}
            handleRenamingFormCancel={handleRenamingRootNodeFormCancel}
          />
        ) : (
          <TreeRootNodeButton
            node={node}
            searchInput={searchInput}
            shouldRenderChildNodes={shouldRenderChildNodes}
            handleRootNodeClick={onRootUpdate}
          />
        )}

        <TreeRootNodeActions
          node={node}
          searchInput={searchInput}
          isRenamingRootNode={isRenamingRootNode}
          // setIsAddingRootFileNode={setIsAddingRootFileNode}
          // setIsAddingRootFolderNode={setIsAddingRootFolderNode}
          setIsRenamingRootNode={setIsRenamingRootNode}
          allFoldersAreCollapsed={allFoldersAreCollapsed}
          allFoldersAreExpanded={allFoldersAreExpanded}
          // handleCollapseAll={handleCollapseAll}
          // handleExpandAll={handleExpandAll}
        />
        {closestEdge && <DropIndicator edge={closestEdge} gap={0} className="z-10" />}
      </div>

      {shouldRenderChildNodes && (
        <TreeRootNodeChildren
          node={node}
          onNodeUpdate={onNodeUpdate}
          // isAddingRootFileNode={isAddingRootFileNode}
          // isAddingRootFolderNode={isAddingRootFolderNode}
          // handleAddFormRootSubmit={handleAddFormRootSubmit}
          // handleAddFormRootCancel={handleAddFormRootCancel}
        />
      )}
    </div>
  );
};
