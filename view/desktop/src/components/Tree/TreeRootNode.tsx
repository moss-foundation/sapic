import { useContext, useEffect, useRef } from "react";

import { Icon, Scrollbar } from "@/lib/ui";
import { cn } from "@/utils";

import { ActionButton, ActionMenu, DropIndicator, TreeContext } from "..";
import { useDraggableRootNode } from "./hooks/useDraggableRootNode";
import { useDropTargetRootNode } from "./hooks/useDropTargetRootNode";
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
    allFoldersAreCollapsed,
    allFoldersAreExpanded,
    searchInput,
    rootOffset,
    nodeOffset,
    onRootAddCallback,
    onRootRenameCallback,
    onRootClickCallback,
    onRootDoubleClickCallback,
  } = useContext(TreeContext);

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

  useDropTargetRootNode(node, treeId, dropTargetFolderRef);

  const shouldRenderChildNodes = !!searchInput || (!searchInput && node.isFolder && node.isExpanded);

  return (
    <div ref={dropTargetFolderRef} className={cn("group relative w-full")}>
      <div
        ref={draggableRootRef}
        className="flex w-full min-w-0 items-center justify-between gap-1 py-[5px] pr-2"
        style={{ paddingLeft: rootOffset, paddingRight: rootOffset }}
      >
        {isRenamingRootNode ? (
          <div className="flex grow cursor-pointer items-center gap-1">
            <Icon
              icon="ChevronRight"
              className={cn("text-(--moss-icon-primary-text)", {
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
            className="flex grow cursor-pointer items-center gap-1 overflow-hidden font-medium"
            onClick={() => {
              handleFolderClick();
              onRootClickCallback?.(node);
            }}
            onDoubleClick={() => onRootDoubleClickCallback?.(node)}
          >
            <Icon
              icon="ChevronRight"
              className={cn("text-(--moss-icon-primary-text)", {
                "rotate-90": shouldRenderChildNodes,
              })}
            />
            <NodeLabel label={node.id} searchInput={searchInput} />
          </button>
        )}

        <div className="flex items-center">
          {node.isExpanded && !searchInput && !isRenamingRootNode && (
            <div
              className={`hidden items-center opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover:flex group-hover:opacity-100`}
            >
              <ActionButton icon="Add" onClick={() => setIsAddingRootFileNode(true)} />
              <ActionButton icon="Refresh" />
              <ActionButton icon="ExpandAll" disabled={allFoldersAreExpanded} onClick={handleExpandAll} />
              <ActionButton icon="CollapseAll" disabled={allFoldersAreCollapsed} onClick={handleCollapseAll} />
            </div>
          )}
          <ActionMenu.Root>
            <ActionMenu.Trigger asChild>
              <ActionButton icon="MoreHorizontal" />
            </ActionMenu.Trigger>
            <ActionMenu.Portal>
              <ActionMenu.Content className="z-30" align="center">
                <ActionMenu.Item onClick={() => setIsAddingRootFileNode(true)}>Add File</ActionMenu.Item>
                <ActionMenu.Item onClick={() => setIsAddingRootFolderNode(true)}>Add Folder</ActionMenu.Item>
                <ActionMenu.Item onClick={() => setIsRenamingRootNode(true)}>Rename...</ActionMenu.Item>
              </ActionMenu.Content>
            </ActionMenu.Portal>
          </ActionMenu.Root>
        </div>
        {closestEdge && <DropIndicator edge={closestEdge} gap={0} className="z-10" />}
      </div>

      {shouldRenderChildNodes && !isRootDragging && (
        <Scrollbar className="h-full w-full">
          <ul className={cn("h-full w-full", { "pb-2": node.childNodes.length > 0 && node.isExpanded })}>
            {filteredChildNodes.map((childNode, index) => (
              <TreeNode
                parentNode={node}
                onNodeUpdate={onNodeUpdate}
                key={childNode.uniqueId}
                node={childNode}
                depth={1}
                isLastChild={index === filteredChildNodes.length - 1}
              />
            ))}
            {(isAddingRootFileNode || isAddingRootFolderNode) && (
              <div className="flex w-full min-w-0 items-center gap-1 py-0.5" style={{ paddingLeft: nodeOffset * 1 }}>
                <TestCollectionIcon type={node.type} className="opacity-0" />
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
