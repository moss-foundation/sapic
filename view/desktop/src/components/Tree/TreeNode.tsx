import { useContext, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

import { ActionMenu, TreeContext } from "..";
import { DropIndicatorWithInstruction } from "./DropIndicatorWithInstruction";
import { useInstructionNode } from "./hooks/useInstructionNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
import { NodeAddForm } from "./NodeAddForm";
import NodeLabel from "./NodeLabel";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TestCollectionIcon } from "./TestCollectionIcon";
import { TreeNodeComponentProps, TreeNodeProps } from "./types";
import { hasDescendantWithSearchInput } from "./utils";

export const TreeNode = ({ node, onNodeUpdate, depth, parentNode }: TreeNodeComponentProps) => {
  const { searchInput, onNodeAddCallback, onNodeRenameCallback } = useContext(TreeContext);

  const dropTargetFolderRef = useRef<HTMLDivElement>(null);
  const dropTargetListRef = useRef<HTMLLIElement>(null);

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useNodeAddForm(node, onNodeUpdate);

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useNodeRenamingForm(
    node,
    onNodeUpdate
  );

  const shouldRenderChildNodes =
    !!searchInput || isAddingFileNode || isAddingFolderNode || (!searchInput && node.isFolder && node.isExpanded);

  return (
    <li ref={dropTargetListRef}>
      {isRenamingNode ? (
        <TreeNodeRenameForm
          node={node}
          depth={depth}
          parentNode={parentNode}
          onNodeRenameCallback={onNodeRenameCallback}
          handleRenamingFormSubmit={handleRenamingFormSubmit}
          handleRenamingFormCancel={handleRenamingFormCancel}
        />
      ) : (
        <TreeNodeButton
          node={node}
          onNodeUpdate={onNodeUpdate}
          depth={depth}
          onAddFile={() => setIsAddingFileNode(true)}
          onAddFolder={() => setIsAddingFolderNode(true)}
          onRename={() => setIsRenamingNode(true)}
        />
      )}

      {(isAddingFileNode || isAddingFolderNode) && (
        <TreeNodeAddForm
          node={node}
          depth={depth}
          isAddingFileNode={isAddingFileNode}
          isAddingFolderNode={isAddingFolderNode}
          onNodeAddCallback={onNodeAddCallback}
          handleAddFormSubmit={handleAddFormSubmit}
          handleAddFormCancel={handleAddFormCancel}
        />
      )}

      {shouldRenderChildNodes && (
        <TreeNodeChildren
          node={node}
          onNodeUpdate={onNodeUpdate}
          depth={depth}
          dropTargetFolderRef={dropTargetFolderRef}
        />
      )}
    </li>
  );
};

const TreeNodeButton = ({ node, onNodeUpdate, depth, onAddFile, onAddFolder, onRename }) => {
  const { treeId, nodeOffset, searchInput, onNodeClickCallback, onNodeDoubleClickCallback } = useContext(TreeContext);

  const draggableNodeRef = useRef<HTMLButtonElement>(null);
  // const { instruction } = useDropTargetNode(node, treeId, draggableNodeRef, depth);

  const [preview, setPreview] = useState<HTMLElement | null>(null);

  // const { isDragging } = useDraggableNode(draggableNodeRef, node, treeId, setPreview);

  const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

  const { instruction, isDragging } = useInstructionNode(node, treeId, draggableNodeRef, depth, setPreview);

  const handleClick = () => {
    if (node.isFolder) {
      onNodeUpdate({
        ...node,
        isExpanded: !node.isExpanded,
      });
    } else {
      addOrFocusPanel({
        id: `${node.id}`,
        params: {
          treeId,
          iconType: node.type,
          workspace: true,
        },
        component: "Default",
      });
    }
    onNodeClickCallback?.(node);
  };

  const handleDoubleClick = () => onNodeDoubleClickCallback?.(node);

  const nodePaddingLeft = depth * nodeOffset;
  const shouldRenderChildNodes = !!searchInput || (!searchInput && node.isFolder && node.isExpanded);

  return (
    <ActionMenu.Root modal={false}>
      <ActionMenu.Trigger openOnRightClick>
        <button
          ref={draggableNodeRef}
          onClick={handleClick}
          onDoubleClick={handleDoubleClick}
          className={cn(
            "group/treeNode relative flex h-full w-full min-w-0 cursor-pointer items-center dark:hover:text-black"
          )}
        >
          <span
            className={cn("absolute inset-x-2 h-full w-[calc(100%-16px)] rounded-sm", {
              "group-hover/treeNode:background-(--moss-secondary-background-hover)":
                !isDragging && activePanelId !== node.id,
              "background-(--moss-info-background-hover)": activePanelId === node.id && node.uniqueId !== "DraggedNode",
            })}
          />

          <span
            className={cn("relative z-10 flex h-full w-full items-center gap-1 py-0.5")}
            style={{ paddingLeft: nodePaddingLeft }}
          >
            {instruction && (
              <DropIndicatorWithInstruction style={{ paddingLeft: nodePaddingLeft }} instruction={instruction} />
            )}
            <Icon
              icon="ChevronRight"
              className={cn("text-(--moss-icon-primary-text)", {
                "rotate-90": shouldRenderChildNodes,
                "opacity-0": !node.isFolder,
              })}
            />
            <TestCollectionIcon type={node.type} />
            <NodeLabel label={node.id} searchInput={searchInput} />
            <span className="DragHandle h-full min-h-4 grow" />
          </span>

          {preview &&
            createPortal(
              <ul className="background-(--moss-primary-background) flex gap-1 rounded-sm">
                <TreeNode
                  parentNode={{
                    uniqueId: "-",
                    childNodes: [],
                    type: "",
                    order: 0,
                    isFolder: false,
                    isExpanded: false,
                    id: "-",
                    isRoot: false,
                  }}
                  node={{ ...node, uniqueId: "DraggedNode", childNodes: [] }}
                  onNodeUpdate={() => {}}
                  depth={0}
                />
                <Icon icon="ChevronRight" className={cn("opacity-0")} />
              </ul>,
              preview
            )}
        </button>
      </ActionMenu.Trigger>
      <ActionMenu.Portal>
        <ActionMenu.Content>
          {node.isFolder && <ActionMenu.Item onClick={onAddFile}>Add File</ActionMenu.Item>}
          {node.isFolder && <ActionMenu.Item onClick={onAddFolder}>Add Folder</ActionMenu.Item>}
          <ActionMenu.Item onClick={onRename}>Edit</ActionMenu.Item>
        </ActionMenu.Content>
      </ActionMenu.Portal>
    </ActionMenu.Root>
  );
};

const TreeNodeRenameForm = ({
  node,
  depth,
  parentNode,
  onNodeRenameCallback,
  handleRenamingFormSubmit,
  handleRenamingFormCancel,
}) => {
  const { nodeOffset, searchInput } = useContext(TreeContext);
  const nodePaddingLeft = depth * nodeOffset;
  const shouldRenderChildNodes = !!searchInput || (!searchInput && node.isFolder && node.isExpanded);

  return (
    <div className="w-full min-w-0">
      <span className="flex w-full items-center gap-1 py-0.5" style={{ paddingLeft: nodePaddingLeft }}>
        <Icon
          icon="ChevronRight"
          className={cn("text-(--moss-icon-primary-text)", {
            "rotate-90": shouldRenderChildNodes,
            "opacity-0": !node.isFolder,
          })}
        />
        <TestCollectionIcon type={node.type} />
        <NodeRenamingForm
          onSubmit={(newName) => {
            handleRenamingFormSubmit(newName);
            onNodeRenameCallback?.({ ...node, id: newName });
          }}
          onCancel={handleRenamingFormCancel}
          restrictedNames={parentNode.childNodes.map((childNode) => childNode.id)}
          currentName={node.id}
        />
      </span>
    </div>
  );
};

const TreeNodeAddForm = ({
  node,
  depth,
  isAddingFileNode,
  isAddingFolderNode,
  onNodeAddCallback,
  handleAddFormSubmit,
  handleAddFormCancel,
}) => {
  const { nodeOffset } = useContext(TreeContext);
  const nodePaddingLeftForAddForm = (depth + 1) * nodeOffset;

  return (
    <div style={{ paddingLeft: nodePaddingLeftForAddForm }} className="flex w-full min-w-0 items-center gap-1">
      <Icon icon="ChevronRight" className={cn("opacity-0")} />
      <TestCollectionIcon
        type={node.type}
        className={cn("ml-auto", {
          "opacity-0": isAddingFileNode,
        })}
      />
      <NodeAddForm
        isFolder={isAddingFolderNode}
        restrictedNames={node.childNodes.map((childNode) => childNode.id)}
        onSubmit={(newNode) => {
          handleAddFormSubmit(newNode);
          onNodeAddCallback?.({ ...node, childNodes: [...node.childNodes, newNode] } as TreeNodeProps);
        }}
        onCancel={handleAddFormCancel}
      />
    </div>
  );
};

const TreeNodeChildren = ({ node, onNodeUpdate, depth, dropTargetFolderRef }) => {
  const { searchInput } = useContext(TreeContext);
  const filteredChildNodes = searchInput
    ? node.childNodes.filter((childNode) => hasDescendantWithSearchInput(childNode, searchInput))
    : node.childNodes;

  return (
    <div className="contents" ref={dropTargetFolderRef}>
      <ul className="h-full">
        {filteredChildNodes.map((childNode) => (
          <TreeNode
            parentNode={node}
            onNodeUpdate={onNodeUpdate}
            key={childNode.uniqueId}
            node={childNode}
            depth={depth + 1}
          />
        ))}
      </ul>
    </div>
  );
};

export default TreeNode;
