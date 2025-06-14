import { useContext, useEffect, useRef } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { ActionButton, ActionMenu, DropIndicator, TreeContext } from "..";
import TestMossImage from "../../assets/images/TestMossImage.webp";
import { useDraggableRootNode } from "./hooks/useDraggableRootNode";
import { useDropTargetRootNode } from "./hooks/useDropTargetRootNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
import { NodeAddForm } from "./NodeAddForm";
import NodeLabel from "./NodeLabel";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TestCollectionIcon } from "./TestCollectionIcon";
import TreeNode from "./TreeNode";
import { NodeProps, TreeNodeProps, TreeRootNodeProps } from "./types";
import { collapseAllNodes, expandAllNodes, hasDescendantWithSearchInput } from "./utils";

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
            "group-hover/TreeRootHeader:background-(--moss-secondary-background-hover) absolute h-[calc(100%-8px)] w-[calc(100%-16px)] rounded-sm"
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

interface TreeRootNodeButtonProps {
  node: TreeNodeProps;
  searchInput?: string;
  shouldRenderChildNodes: boolean;
  handleFolderClick: () => void;
}
const TreeRootNodeButton = ({
  node,
  searchInput,
  shouldRenderChildNodes,
  handleFolderClick,
}: TreeRootNodeButtonProps) => {
  const { onRootClickCallback, onRootDoubleClickCallback } = useContext(TreeContext);

  return (
    <button
      className="group/treeRootNodeTrigger relative flex grow cursor-pointer items-center gap-1.5 overflow-hidden font-medium"
      onClick={() => {
        handleFolderClick();
        onRootClickCallback?.(node);
      }}
      onDoubleClick={() => onRootDoubleClickCallback?.(node)}
    >
      <span className="flex size-5 shrink-0 items-center justify-center">
        <Icon
          icon="ChevronRight"
          className={cn("hidden text-(--moss-icon-primary-text) group-hover/treeRootNodeTrigger:block", {
            "rotate-90": shouldRenderChildNodes,
          })}
        />
        {/* TODO: Replace with the actual image and don't forget to remove image from assets */}
        <div className="h-full w-full rounded outline-1 outline-(--moss-border-color) group-hover/treeRootNodeTrigger:hidden">
          <img src={TestMossImage} className="h-full w-full" />
        </div>
      </span>
      <NodeLabel label={node.id} searchInput={searchInput} />
    </button>
  );
};

interface TreeRootNodeActionsProps {
  node: TreeNodeProps;
  searchInput?: string;
  isRenamingRootNode: boolean;
  setIsAddingRootFileNode: (isAdding: boolean) => void;
  setIsAddingRootFolderNode: (isAdding: boolean) => void;
  setIsRenamingRootNode: (isRenaming: boolean) => void;
  allFoldersAreCollapsed: boolean;
  allFoldersAreExpanded: boolean;
  handleCollapseAll: () => void;
  handleExpandAll: () => void;
}
const TreeRootNodeActions = ({
  node,
  searchInput,
  isRenamingRootNode,
  setIsAddingRootFileNode,
  setIsAddingRootFolderNode,
  setIsRenamingRootNode,
  allFoldersAreCollapsed,
  allFoldersAreExpanded,
  handleCollapseAll,
  handleExpandAll,
}: TreeRootNodeActionsProps) => {
  return (
    <div className="z-10 flex items-center">
      {node.isExpanded && !searchInput && !isRenamingRootNode && (
        <div
          className={`hidden items-center opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover:flex group-hover:opacity-100`}
        >
          <ActionButton icon="Add" onClick={() => setIsAddingRootFileNode(true)} />
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
            <ActionMenu.Item>Refresh</ActionMenu.Item>
            <ActionMenu.Item disabled={allFoldersAreExpanded} onClick={handleExpandAll}>
              ExpandAll
            </ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    </div>
  );
};

interface TreeRootNodeRenameFormProps {
  node: TreeNodeProps;
  handleRenamingFormSubmit: (newName: string) => void;
  handleRenamingFormCancel: () => void;
}

const TreeRootNodeRenameForm = ({
  node,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}: TreeRootNodeRenameFormProps) => {
  const { onRootRenameCallback } = useContext(TreeContext);

  return (
    <div className="flex grow cursor-pointer items-center gap-1.5">
      {/* TODO: Replace with the actual image and don't forget to remove image from assets */}
      <div className="flex size-5 shrink-0 items-center justify-center rounded outline-1 outline-(--moss-border-color)">
        <img src={TestMossImage} className="h-full w-full" />
      </div>
      <NodeRenamingForm
        onSubmit={(newName) => {
          handleRenamingFormSubmit(newName);
          onRootRenameCallback?.({ ...node, id: newName });
        }}
        onCancel={handleRenamingFormCancel}
        currentName={node.id}
      />
    </div>
  );
};

interface TreeRootNodeChildrenProps {
  node: TreeNodeProps;
  onNodeUpdate: (node: TreeNodeProps) => void;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleAddFormRootSubmit: (newNode: NodeProps) => void;
  handleAddFormRootCancel: () => void;
}

const TreeRootNodeChildren = ({
  node,
  onNodeUpdate,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleAddFormRootSubmit,
  handleAddFormRootCancel,
}: TreeRootNodeChildrenProps) => {
  const { searchInput, nodeOffset, onRootAddCallback } = useContext(TreeContext);

  const { isDragging: isRootDragging } = useDraggableRootNode(
    useRef<HTMLDivElement>(null),
    node,
    useContext(TreeContext).treeId,
    false
  );

  const filteredChildNodes = searchInput
    ? node.childNodes.filter((childNode) => hasDescendantWithSearchInput(childNode, searchInput))
    : node.childNodes;

  if (isRootDragging) return null;

  return (
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
  );
};
