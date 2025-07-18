import { useContext, useRef } from "react";

import { useStreamedCollections } from "@/hooks";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { cn } from "@/utils";

import { DropIndicatorWithInstruction } from "../DropIndicatorWithInstruction";
import { useDraggableRootNode } from "../hooks/useDraggableRootNode";
import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";
import { getAllNestedEntries } from "../utils2";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootNodeActions } from "./TreeRootNodeActions";
import { TreeRootNodeButton } from "./TreeRootNodeButton";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootNodeRenameForm } from "./TreeRootNodeRenameForm";
import { calculateShouldRenderRootChildNodes } from "./utils";

export interface TreeRootNodeProps {
  node: TreeCollectionRootNode;
}

export const TreeRootNode = ({ node }: TreeRootNodeProps) => {
  const { searchInput, rootOffset } = useContext(TreeContext);
  const { data: streamedCollections } = useStreamedCollections();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetRootRef = useRef<HTMLDivElement>(null);

  const handleExpandAll = () => {
    // const newNode = expandAllNodes(node);
  };

  const handleCollapseAll = async () => {
    const requestsEntries = getAllNestedEntries(node.requests);
    const endpointsEntries = getAllNestedEntries(node.endpoints);
    const componentsEntries = getAllNestedEntries(node.components);
    const schemasEntries = getAllNestedEntries(node.schemas);

    const entriesToUpdate = [...requestsEntries, ...endpointsEntries, ...componentsEntries, ...schemasEntries].filter(
      (entry) => entry.expanded
    );

    const inputEntries = entriesToUpdate.map((entry) => {
      if (entry.kind === "Dir") {
        return {
          DIR: {
            id: entry.id,
            expanded: false,
          },
        };
      } else {
        return {
          ITEM: {
            id: entry.id,
            expanded: false,
          },
        };
      }
    });

    await batchUpdateCollectionEntry({
      collectionId: node.id,
      entries: {
        entries: inputEntries,
      },
    });
  };

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

  const { instruction, isDragging, canDrop } = useDraggableRootNode(draggableRootRef, node, isRenamingRootNode);

  const shouldRenderRootChildNodes = calculateShouldRenderRootChildNodes(
    node,
    isDragging,
    isAddingRootNodeFile,
    isRenamingRootNode
  );
  console.log({
    shouldRenderRootChildNodes,
  });

  const restrictedNames = streamedCollections?.map((collection) => collection.name) ?? [];

  return (
    <div
      ref={dropTargetRootRef}
      className={cn("group relative w-full", {
        "hidden": isDragging,
      })}
    >
      {instruction && <DropIndicatorWithInstruction instruction={instruction} gap={0} className="" canDrop={canDrop} />}
      <div
        ref={draggableRootRef}
        className="group/TreeRootHeader relative flex w-full min-w-0 items-center justify-between gap-1 py-[3px] pr-2"
        style={{ paddingLeft: rootOffset, paddingRight: rootOffset }}
      >
        <span
          className={cn(
            "group-hover/TreeRootHeader:background-(--moss-secondary-background-hover) absolute inset-x-1 h-[calc(100%-5px)] w-[calc(100%-8px)] rounded-sm",
            {
              "group-hover/TreeRootHeader:background-transparent": isRenamingRootNode,
            }
          )}
        />

        {isRenamingRootNode ? (
          <TreeRootNodeRenameForm
            node={node}
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
          handleCollapseAll={handleCollapseAll}
          handleExpandAll={handleExpandAll}
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
