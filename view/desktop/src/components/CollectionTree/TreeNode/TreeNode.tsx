import { useContext, useRef, useState } from "react";

import { TreeContext } from "../..";
import { AddingFormDivider } from "../AddingFormDivider";
import { DropIndicatorWithInstruction } from "../DropIndicatorWithInstruction";
import { useAddNodeWithDivider } from "../hooks/useAddNodeWithDivider";
import { useInstructionNode } from "../hooks/useInstructionNode";
import { useNodeAddForm } from "../hooks/useNodeAddForm";
import { useNodeRenamingForm } from "../hooks/useNodeRenamingForm";
import { TreeNodeComponentProps, TreeNodeProps } from "../types";
import TreeNodeAddForm from "./TreeNodeAddForm";
import TreeNodeButton from "./TreeNodeButton";
import TreeNodeChildren from "./TreeNodeChildren";
import TreeNodeRenameForm from "./TreeNodeRenameForm";

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

export default TreeNode;
