import { useContext, useRef } from "react";

import { DropIndicator } from "@/components/DropIndicator";
import { useStreamedCollectionEntries, useStreamedCollections } from "@/hooks";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { cn } from "@/utils";

import { useDraggableRootNode } from "../hooks/useDraggableRootNode";
import { useDropTargetRootNode } from "../hooks/useDropTargetRootNode";
import { TreeContext } from "../Tree";
import { TreeCollectionNode, TreeCollectionRootNode } from "../types";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";
import { TreeRootNodeActions } from "./TreeRootNodeActions";
import { TreeRootNodeButton } from "./TreeRootNodeButton";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootNodeRenameForm } from "./TreeRootNodeRenameForm";
import { calculateShouldRenderRootChildNodes } from "./utils";

export interface TreeRootNodeProps {
  onNodeUpdate: (node: TreeCollectionNode) => void;
  onRootNodeUpdate: (node: TreeCollectionRootNode) => void;
  node: TreeCollectionRootNode;
}

export const TreeRootNode = ({ node, onNodeUpdate, onRootNodeUpdate }: TreeRootNodeProps) => {
  const { id, allFoldersAreCollapsed, allFoldersAreExpanded, searchInput, rootOffset } = useContext(TreeContext);
  const { data: streamedCollections } = useStreamedCollections();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();
  const { data: streamedCollectionEntries } = useStreamedCollectionEntries(id);

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetRootRef = useRef<HTMLDivElement>(null);

  const handleExpandAll = () => {
    if (!streamedCollectionEntries) return;

    const entriesToExpand = streamedCollectionEntries
      ?.filter((entry) => entry.kind === "Dir" && entry.expanded === false)
      .map((entry) => ({
        DIR: {
          id: entry.id,
          expanded: true,
          path: entry.path.raw,
        },
      }));

    batchUpdateCollectionEntry({
      collectionId: id,
      entries: {
        entries: entriesToExpand,
      },
    });
  };

  const handleCollapseAll = () => {
    if (!streamedCollectionEntries) return;

    const entriesToCollapse = streamedCollectionEntries
      ?.filter((entry) => entry.kind === "Dir" && entry.expanded === true)
      .map((entry) => ({
        DIR: {
          id: entry.id,
          expanded: false,
          path: entry.path.raw,
        },
      }));

    batchUpdateCollectionEntry({
      collectionId: id,
      entries: {
        entries: entriesToCollapse,
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
  } = useRootNodeAddForm(node, onRootNodeUpdate);

  const {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  } = useRootNodeRenamingForm(node, onRootNodeUpdate);

  const { closestEdge, isDragging } = useDraggableRootNode(draggableRootRef, node, id, isRenamingRootNode);
  useDropTargetRootNode(node, id, dropTargetRootRef);

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

  const shouldRenderRootChildNodes = calculateShouldRenderRootChildNodes(
    node,
    isDragging,
    isAddingRootNodeFile,
    isRenamingRootNode
  );

  const restrictedNames = streamedCollections?.map((collection) => collection.name) ?? [];

  return (
    <div
      ref={dropTargetRootRef}
      className={cn("group relative w-full", {
        "hidden": isDragging,
      })}
    >
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
            onRootNodeClick={onRootNodeUpdate}
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

      {shouldRenderRootChildNodes && (
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
