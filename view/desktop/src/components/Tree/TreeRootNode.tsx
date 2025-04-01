import { useContext, useEffect, useRef } from "react";

import { cn } from "@/utils";

import { DropdownMenu, DropIndicator, Icon, Scrollbar, TreeContext } from "..";
import { useDraggableRootNode } from "./hooks/useDraggableRootNode";
import { useDropTargetNode } from "./hooks/useDropTargetNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
import { NodeAddForm } from "./NodeAddForm";
import NodeLabel from "./NodeLabel";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TestCollectionIcon } from "./TestCollectionIcon";
import TreeNode from "./TreeNode";
import { TreeNodeProps, TreeRootNodeProps } from "./types";
import { collapseAllNodes, expandAllNodes, hasDescendantWithSearchInput } from "./utils";

export const TreeRootNode = ({ node, onNodeUpdate }: TreeRootNodeProps) => {
  const {
    treeId,
    paddingLeft,
    paddingRight,
    allFoldersAreCollapsed,
    allFoldersAreExpanded,
    searchInput,

    onRootAddCallback,
    onRootRenameCallback,
    onRootClickCallback,
    onRootDoubleClickCallback,
  } = useContext(TreeContext);

  const shouldRenderChildNodes = !!searchInput || (!searchInput && node.isFolder && node.isExpanded);

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

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetFolderRef = useRef<HTMLDivElement>(null);
  const dropTargetListRef = useRef<HTMLLIElement>(null);

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

  const { closestEdge, isDragging: isRootDragging } = useDraggableRootNode(
    draggableRootRef,
    node,
    treeId,
    isRenamingRootNode
  );

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

  const filteredChildNodes = searchInput
    ? node.childNodes.filter((childNode) => hasDescendantWithSearchInput(childNode, searchInput))
    : node.childNodes;
  useDropTargetNode(node, treeId, dropTargetListRef, dropTargetFolderRef);

  return (
    <div ref={dropTargetFolderRef} className={cn("group relative w-full")}>
      <div
        ref={draggableRootRef}
        className="focus-within:background-(--moss-treeNode-bg) flex w-full min-w-0 items-center justify-between gap-1 py-[7px] pr-2"
        style={{ paddingLeft, paddingRight }}
      >
        {isRenamingRootNode ? (
          <div className="flex grow cursor-pointer items-center gap-1">
            <Icon
              icon="TreeChevronRightIcon"
              className={cn("text-[#717171]", {
                "rotate-90": shouldRenderChildNodes,
              })}
            />
            <NodeRenamingForm
              onSubmit={(newName) => {
                handleRenamingRootFormSubmit(newName);
                onRootRenameCallback?.({ ...node, id: newName });
              }}
              onCancel={handleRenamingRootFormCancel}
              currentName={node.id}
            />
          </div>
        ) : (
          <button
            className="flex grow cursor-pointer items-center gap-1 overflow-hidden"
            onClick={() => {
              handleFolderClick();
              onRootClickCallback?.(node);
            }}
            onDoubleClick={() => onRootDoubleClickCallback?.(node)}
          >
            <Icon
              icon="TreeChevronRightIcon"
              className={cn("text-[#717171]", {
                "rotate-90": shouldRenderChildNodes,
              })}
            />
            <NodeLabel label={node.id} searchInput={searchInput} />
          </button>
        )}

        <div className="flex items-center gap-1">
          {node.isExpanded && !searchInput && (
            <div className="flex items-center gap-1 opacity-0 transition-opacity duration-100 group-hover:opacity-100">
              <button
                disabled={allFoldersAreExpanded}
                className={`disabled:hover:background-transparent disabled:hover:dark:background-transparent background-(--moss-icon-primary-bg) hover:background-(--moss-icon-primary-bg-hover) flex size-[22px] cursor-pointer items-center justify-center rounded-[3px] text-(--moss-icon-primary-text) disabled:cursor-default disabled:opacity-50 disabled:hover:text-(--moss-icon-primary-text)`}
                onClick={handleExpandAll}
              >
                <Icon icon="TreeExpandAllIcon" />
              </button>
              <button
                disabled={allFoldersAreCollapsed}
                className={`disabled:hover:background-transparent disabled:hover:dark:background-transparent background-(--moss-icon-primary-bg) hover:background-(--moss-icon-primary-bg-hover) flex size-[22px] cursor-pointer items-center justify-center rounded-[3px] text-(--moss-icon-primary-text) disabled:cursor-default disabled:opacity-50 disabled:hover:text-(--moss-icon-primary-text)`}
                onClick={handleCollapseAll}
              >
                <Icon icon="TreeCollapseAllIcon" />
              </button>
            </div>
          )}
          <DropdownMenu.Root>
            <DropdownMenu.Trigger className="background-(--moss-icon-primary-bg) hover:background-(--moss-icon-primary-bg-hover) flex size-[22px] cursor-pointer items-center justify-center rounded-[3px] text-(--moss-icon-primary-text) disabled:cursor-default disabled:opacity-50 disabled:hover:text-(--moss-icon-primary-text)">
              <Icon icon="TreeDetailIcon" />
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
              <DropdownMenu.Content className="z-30">
                <DropdownMenu.Item label="Add File" onClick={() => setIsAddingRootFileNode(true)} />
                <DropdownMenu.Item label="Add Folder" onClick={() => setIsAddingRootFolderNode(true)} />
                <DropdownMenu.Item label="Rename..." onClick={() => setIsRenamingRootNode(true)} />
              </DropdownMenu.Content>
            </DropdownMenu.Portal>
          </DropdownMenu.Root>
        </div>
        {closestEdge && <DropIndicator edge={closestEdge} gap={0} className="z-10" />}
      </div>

      {shouldRenderChildNodes && !isRootDragging && (
        <Scrollbar className="h-full w-full">
          <ul className={cn("h-full w-full", { "pb-2": node.childNodes.length > 0 && node.isExpanded })}>
            {filteredChildNodes.map((childNode) => (
              <TreeNode
                parentNode={node}
                onNodeUpdate={onNodeUpdate}
                key={childNode.uniqueId}
                node={childNode}
                depth={0}
              />
            ))}
            {(isAddingRootFileNode || isAddingRootFolderNode) && (
              <div
                className="flex w-full min-w-0 items-center gap-1 py-0.5"
                style={{ paddingLeft: `${paddingLeft + 4}px` }}
              >
                <TestCollectionIcon type={node.type} className={cn({ "opacity-0": isAddingRootFileNode })} />
                <NodeAddForm
                  isFolder={isAddingRootFolderNode}
                  restrictedNames={node.childNodes.map((childNode) => childNode.id)}
                  onSubmit={(newNode) => {
                    handleAddFormRootSubmit(newNode);
                    onRootAddCallback?.({ ...node, childNodes: [...node.childNodes, newNode] } as TreeNodeProps);
                  }}
                  onCancel={handleAddFormRootCancel}
                />
              </div>
            )}
          </ul>
        </Scrollbar>
      )}
    </div>
  );
};
