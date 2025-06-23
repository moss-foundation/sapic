import { useContext, useRef } from "react";

import { DropIndicator } from "@/components/DropIndicator";
import { cn } from "@/utils";

import { useDraggableRootNode } from "../hooks/useDraggableRootNode";
import { useDropTargetRootNode } from "../hooks/useDropTargetRootNode";
import { TreeContext } from "../Tree";
import { TreeCollectionNode, TreeCollectionRootNode } from "../types";
import { collapseAllNodes, expandAllNodes } from "../utils/TreeRootUtils";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootNodeActions } from "./TreeRootNodeActions";
import { TreeRootNodeButton } from "./TreeRootNodeButton";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootNodeRenameForm } from "./TreeRootNodeRenameForm";

const shouldRenderRootChildNodes = (
  node: TreeCollectionRootNode,
  isDragging: boolean,
  isAddingRootNodeFile: boolean,
  isRenamingRootNode: boolean
) => {
  if (!node.expanded) {
    return false;
  }

  if (isDragging) {
    return false;
  }

  if (isAddingRootNodeFile || isRenamingRootNode) {
    return true;
  }

  return true;
};

export interface TreeRootNodeProps {
  onNodeUpdate: (node: TreeCollectionNode) => void;
  onRootUpdate: (node: TreeCollectionRootNode) => void;
  node: TreeCollectionRootNode;
}

export const TreeRootNode = ({ node, onNodeUpdate, onRootUpdate }: TreeRootNodeProps) => {
  const { treeId, allFoldersAreCollapsed, allFoldersAreExpanded, searchInput, rootOffset } = useContext(TreeContext);

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetRootRef = useRef<HTMLDivElement>(null);

  const handleExpandAll = () => {
    const newNode = expandAllNodes(node);
    onRootUpdate(newNode);
  };

  const handleCollapseAll = () => {
    const newNode = collapseAllNodes(node);
    onRootUpdate(newNode);
  };

  const {
    isAddingRootNodeFile,
    isAddingRootNodeFolder,
    setIsAddingRootNodeFile,
    setIsAddingRootNodeFolder,
    handleRootAddFormCancel,
    handleRootAddFormSubmit,
  } = useRootNodeAddForm(node, onRootUpdate);

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

  const shouldRenderChildNodes = shouldRenderRootChildNodes(node, isDragging, isAddingRootNodeFile, isRenamingRootNode);

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
            "group-hover/TreeRootHeader:background-(--moss-secondary-background-hover) absolute inset-x-1 h-[calc(100%-8px)] w-[calc(100%-8px)] rounded-sm",
            {
              "group-hover/TreeRootHeader:background-transparent": isRenamingRootNode,
            }
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
          setIsAddingRootFileNode={setIsAddingRootNodeFile}
          setIsAddingRootFolderNode={setIsAddingRootNodeFolder}
          setIsRenamingRootNode={setIsRenamingRootNode}
          allFoldersAreCollapsed={allFoldersAreCollapsed}
          allFoldersAreExpanded={allFoldersAreExpanded}
          handleCollapseAll={handleCollapseAll}
          handleExpandAll={handleExpandAll}
        />
      </div>
      {closestEdge && <DropIndicator edge={closestEdge} gap={0} className="z-10" />}

      {shouldRenderChildNodes && (
        <TreeRootNodeChildren
          node={node}
          onNodeUpdate={onNodeUpdate}
          isAddingRootFileNode={isAddingRootNodeFile}
          isAddingRootFolderNode={isAddingRootNodeFolder}
          handleAddFormRootSubmit={handleRootAddFormSubmit}
          handleAddFormRootCancel={handleRootAddFormCancel}
        />
      )}
    </div>
  );
};
