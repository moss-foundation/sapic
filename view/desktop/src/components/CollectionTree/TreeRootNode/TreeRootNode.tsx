import { useContext, useEffect, useRef } from "react";

import DropIndicator from "@/components/DropIndicator";
import { cn } from "@/utils";

import { useDraggableRootNode } from "../hooks/useDraggableRootNode";
import { useDropTargetRootNode } from "../hooks/useDropTargetRootNode";
import { useNodeAddForm } from "../hooks/useNodeAddForm";
import { useNodeRenamingForm } from "../hooks/useNodeRenamingForm";
import { TreeContext } from "../Tree";
import { TreeRootNodeProps } from "../types";
import { collapseAllNodes, expandAllNodes } from "../utils";
import { TreeRootNodeActions } from "./TreeRootNodeActions";
import { TreeRootNodeButton } from "./TreeRootNodeButton";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootNodeRenameForm } from "./TreeRootNodeRenameForm";

export const TreeRootNode = ({ node, onNodeUpdate }: TreeRootNodeProps) => {
  const { treeId, allFoldersAreCollapsed, allFoldersAreExpanded, searchInput, rootOffset } = useContext(TreeContext);

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetFolderRef = useRef<HTMLDivElement>(null);

  const handleExpandAll = () => {
    const newNode = expandAllNodes(node);
    onNodeUpdate({
      ...node,
      childNodes: newNode.childNodes,
    });
  };

  const handleCollapseAll = () => {
    const newNode = collapseAllNodes(node);
    onNodeUpdate({
      ...node,
      childNodes: newNode.childNodes,
    });
  };

  const handleFolderClick = () => {
    if (!node.isFolder || searchInput) return;
    onNodeUpdate({
      ...node,
      isExpanded: !node.isExpanded,
    });
  };

  const {
    isAddingFileNode: isAddingRootFileNode,
    isAddingFolderNode: isAddingRootFolderNode,
    setIsAddingFileNode: setIsAddingRootFileNode,
    setIsAddingFolderNode: setIsAddingRootFolderNode,
    handleAddFormSubmit: handleAddFormRootSubmit,
    handleAddFormCancel: handleAddFormRootCancel,
  } = useNodeAddForm(node, onNodeUpdate);

  const {
    isRenamingNode: isRenamingRootNode,
    setIsRenamingNode: setIsRenamingRootNode,
    handleRenamingFormSubmit: handleRenamingRootFormSubmit,
    handleRenamingFormCancel: handleRenamingRootFormCancel,
  } = useNodeRenamingForm(node, onNodeUpdate);

  const { closestEdge, isDragging } = useDraggableRootNode(draggableRootRef, node, treeId, isRenamingRootNode);

  useEffect(() => {
    const handleNewCollectionWasCreated = (event: Event) => {
      const customEvent = event as CustomEvent<{ treeId: string }>;
      if (treeId === customEvent.detail.treeId) {
        setIsRenamingRootNode(true);
      }
    };
    window.addEventListener("newCollectionWasCreated", handleNewCollectionWasCreated);
    return () => {
      window.removeEventListener("newCollectionWasCreated", handleNewCollectionWasCreated as EventListener);
    };
  }, [setIsRenamingRootNode, treeId]);

  useDropTargetRootNode(node, treeId, dropTargetFolderRef);

  const shouldRenderChildNodes =
    !!searchInput ||
    (!searchInput && node.isFolder && node.isExpanded) ||
    isAddingRootFileNode ||
    isAddingRootFolderNode;

  return (
    <div ref={dropTargetFolderRef} className={cn("group relative w-full")}>
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
            handleRenamingFormSubmit={handleRenamingRootFormSubmit}
            handleRenamingFormCancel={handleRenamingRootFormCancel}
          />
        ) : (
          <TreeRootNodeButton
            node={node}
            searchInput={searchInput}
            shouldRenderChildNodes={shouldRenderChildNodes}
            handleFolderClick={handleFolderClick}
          />
        )}

        <TreeRootNodeActions
          node={node}
          searchInput={searchInput}
          isRenamingRootNode={isRenamingRootNode}
          setIsAddingRootFileNode={setIsAddingRootFileNode}
          setIsAddingRootFolderNode={setIsAddingRootFolderNode}
          setIsRenamingRootNode={setIsRenamingRootNode}
          allFoldersAreCollapsed={allFoldersAreCollapsed}
          allFoldersAreExpanded={allFoldersAreExpanded}
          handleCollapseAll={handleCollapseAll}
          handleExpandAll={handleExpandAll}
        />
        {closestEdge && <DropIndicator edge={closestEdge} gap={0} className="z-10" />}
      </div>

      {shouldRenderChildNodes && !isDragging && (
        <TreeRootNodeChildren
          node={node}
          onNodeUpdate={onNodeUpdate}
          isAddingRootFileNode={isAddingRootFileNode}
          isAddingRootFolderNode={isAddingRootFolderNode}
          handleAddFormRootSubmit={handleAddFormRootSubmit}
          handleAddFormRootCancel={handleAddFormRootCancel}
        />
      )}
    </div>
  );
};
