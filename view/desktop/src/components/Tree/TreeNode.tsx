import { forwardRef, useContext, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ActionMenu, TreeContext } from "..";
import { DragHandleButton } from "../DragHandleButton";
import { AddingFormDivider } from "./AddingFormDivider";
import { DropIndicatorWithInstruction } from "./DropIndicatorWithInstruction";
import { useAddNodeWithDivider } from "./hooks/useAddNodeWithDivider";
import { useInstructionNode } from "./hooks/useInstructionNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
import { NodeAddForm } from "./NodeAddForm";
import { NodeLabel } from "./NodeLabel";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TestCollectionIcon } from "./TestCollectionIcon";
import { TreeNodeComponentProps, TreeNodeProps } from "./types";
import { hasDescendantWithSearchInput } from "./utils";

const shouldRenderTreeNode = (
  node: TreeNodeProps,
  searchInput: string | undefined,
  isAddingFileNode: boolean,
  isAddingFolderNode: boolean
) => {
  if (isAddingFileNode || isAddingFolderNode) return true;

  if (searchInput) return true;

  if (node.isFolder && node.isExpanded) return true;

  return false;
};

export const TreeNode = ({ node, onNodeUpdate, depth, parentNode, isLastChild }: TreeNodeComponentProps) => {
  const { searchInput, onNodeAddCallback, onNodeRenameCallback, treeId, nodeOffset, paddingRight } =
    useContext(TreeContext);

  const triggerRef = useRef<HTMLButtonElement>(null);

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useNodeAddForm(node, onNodeUpdate);

  const {
    isAddingDividerNode: isAddingDividerNodeAbove,
    setIsAddingDividerNode: setIsAddingDividerNodeAbove,
    handleAddDividerFormSubmit: handleAddDividerFormSubmitAbove,
    handleAddDividerFormCancel: handleAddDividerFormCancelAbove,
  } = useAddNodeWithDivider(parentNode, onNodeUpdate, node.order - 1);

  const {
    isAddingDividerNode: isAddingDividerNodeBelow,
    setIsAddingDividerNode: setIsAddingDividerNodeBelow,
    handleAddDividerFormSubmit: handleAddDividerFormSubmitBelow,
    handleAddDividerFormCancel: handleAddDividerFormCancelBelow,
  } = useAddNodeWithDivider(parentNode, onNodeUpdate, node.order + 1);

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useNodeRenamingForm(
    node,
    onNodeUpdate
  );

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const { instruction, isDragging, canDrop } = useInstructionNode(node, treeId, triggerRef, isLastChild, setPreview);

  const shouldRenderChildNodes = shouldRenderTreeNode(node, searchInput, isAddingFileNode, isAddingFolderNode);
  const nodePaddingLeft = depth * nodeOffset;

  return (
    <li className="relative">
      {node.isFolder && instruction !== null && canDrop === true && (
        <DropIndicatorWithInstruction
          paddingLeft={nodePaddingLeft}
          paddingRight={paddingRight}
          instruction={instruction}
          isFolder={node.isFolder}
          depth={depth}
          isLastChild={isLastChild}
        />
      )}
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
        <>
          <AddingFormDivider
            paddingLeft={nodePaddingLeft}
            paddingRight={paddingRight}
            position="top"
            onClick={() => setIsAddingDividerNodeAbove(true)}
          />

          {isAddingDividerNodeAbove && (
            <TreeNodeAddForm
              node={node}
              depth={depth - 1}
              isAddingFileNode={true}
              isAddingFolderNode={false}
              onNodeAddCallback={onNodeAddCallback}
              handleAddFormSubmit={handleAddDividerFormSubmitAbove}
              handleAddFormCancel={handleAddDividerFormCancelAbove}
            />
          )}

          <TreeNodeButton
            ref={triggerRef}
            node={node}
            onNodeUpdate={onNodeUpdate}
            depth={depth}
            onAddFile={() => setIsAddingFileNode(true)}
            onAddFolder={() => setIsAddingFolderNode(true)}
            onRename={() => setIsRenamingNode(true)}
            isDragging={isDragging}
            canDrop={canDrop}
            instruction={instruction}
            preview={preview}
            isLastChild={isLastChild}
          />

          {isAddingDividerNodeBelow && (
            <TreeNodeAddForm
              node={node}
              depth={depth - 1}
              isAddingFileNode={true}
              isAddingFolderNode={false}
              onNodeAddCallback={onNodeAddCallback}
              handleAddFormSubmit={handleAddDividerFormSubmitBelow}
              handleAddFormCancel={handleAddDividerFormCancelBelow}
            />
          )}

          {isLastChild && (
            <AddingFormDivider
              paddingLeft={nodePaddingLeft}
              paddingRight={paddingRight}
              position="bottom"
              onClick={() => setIsAddingDividerNodeBelow(true)}
            />
          )}
        </>
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

      {shouldRenderChildNodes && <TreeNodeChildren node={node} onNodeUpdate={onNodeUpdate} depth={depth} />}
    </li>
  );
};

interface TreeNodeButtonProps {
  node: TreeNodeProps;
  onNodeUpdate: (node: TreeNodeProps) => void;
  depth: number;
  onAddFile: () => void;
  onAddFolder: () => void;
  onRename: () => void;
  isDragging: boolean;
  canDrop: boolean | null;
  instruction: Instruction | null;
  preview: HTMLElement | null;
  isLastChild: boolean;
}

const TreeNodeButton = forwardRef<HTMLButtonElement, TreeNodeButtonProps>(
  (
    {
      node,
      onNodeUpdate,
      depth,
      onAddFile,
      onAddFolder,
      onRename,
      isDragging,
      canDrop,
      instruction,
      preview,
      isLastChild,
    },
    ref
  ) => {
    const { treeId, nodeOffset, searchInput, onNodeClickCallback, onNodeDoubleClickCallback, paddingRight } =
      useContext(TreeContext);

    const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

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
        <ActionMenu.Trigger asChild openOnRightClick>
          <button
            ref={ref}
            onClick={handleClick}
            onDoubleClick={handleDoubleClick}
            className={cn("group/treeNode relative flex h-full w-full min-w-0 cursor-pointer items-center")}
          >
            <span
              className={cn("absolute inset-x-2 h-full w-[calc(100%-16px)] rounded-sm", {
                "group-hover/treeNode:background-(--moss-secondary-background-hover)":
                  !isDragging && activePanelId !== node.id,
                "background-(--moss-info-background-hover)":
                  activePanelId === node.id && node.uniqueId !== "DraggedNode",
              })}
            />
            <span
              className={cn("relative z-10 flex h-full w-full items-center gap-1 py-0.5", {
                "background-(--moss-error-background)": canDrop === false,
              })}
              style={{ paddingLeft: nodePaddingLeft }}
            >
              <DragHandleButton
                className="absolute top-1/2 left-[1px] -translate-y-1/2 opacity-0 transition-all duration-0 group-hover/treeNode:opacity-100 group-hover/treeNode:delay-400 group-hover/treeNode:duration-150"
                slim
              />

              {!node.isFolder && instruction !== null && canDrop === true && (
                <DropIndicatorWithInstruction
                  paddingLeft={nodePaddingLeft}
                  paddingRight={paddingRight}
                  instruction={instruction}
                  isFolder={node.isFolder}
                  depth={depth}
                  isLastChild={isLastChild}
                />
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
                    isLastChild={false}
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
  }
);

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

const TreeNodeChildren = ({ node, onNodeUpdate, depth }) => {
  const { searchInput } = useContext(TreeContext);
  const filteredChildNodes = searchInput
    ? node.childNodes.filter((childNode) => hasDescendantWithSearchInput(childNode, searchInput))
    : node.childNodes;

  return (
    <div className="contents">
      <ul className="h-full">
        {filteredChildNodes.map((childNode, index) => (
          <TreeNode
            parentNode={node}
            onNodeUpdate={onNodeUpdate}
            key={childNode.uniqueId}
            node={childNode}
            depth={depth + 1}
            isLastChild={index === filteredChildNodes.length - 1}
          />
        ))}
      </ul>
    </div>
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

export default TreeNode;
