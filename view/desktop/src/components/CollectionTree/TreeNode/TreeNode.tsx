import { useContext, useRef, useState } from "react";

import { cn } from "@/utils";

import { TreeContext } from "../..";
import { useDeleteAndUpdatePeers } from "../actions/useDeleteAndUpdatePeers";
import { DropIndicatorWithInstruction } from "../DropIndicatorWithInstruction";
import { useInstructionNode } from "../hooks/useInstructionNode";
import { useNodeAddForm } from "../hooks/useNodeAddForm";
import { useNodeRenamingForm } from "../hooks/useNodeRenamingForm";
import { TreeCollectionNode } from "../types";
import TreeNodeAddForm from "./TreeNodeAddForm";
import TreeNodeButton from "./TreeNodeButton";
import TreeNodeChildren from "./TreeNodeChildren";
import TreeNodeRenameForm from "./TreeNodeRenameForm";

const shouldRenderTreeNode = (
  node: TreeCollectionNode,
  searchInput: string | undefined,
  isAddingFileNode: boolean,
  isAddingFolderNode: boolean
) => {
  if (isAddingFileNode || isAddingFolderNode) return true;

  if (searchInput) return true;

  if (node.kind === "Dir" && node.expanded) return true;

  return false;
};

export interface TreeNodeComponentProps {
  node: TreeCollectionNode;
  depth: number;
  parentNode: TreeCollectionNode;
  isLastChild: boolean;
  isRootNode?: boolean;
}

export const TreeNode = ({ node, depth, parentNode, isLastChild, isRootNode = false }: TreeNodeComponentProps) => {
  const { nodeOffset, paddingRight, id } = useContext(TreeContext);

  const triggerRef = useRef<HTMLButtonElement>(null);

  const { deleteAndUpdatePeers } = useDeleteAndUpdatePeers(id, node, parentNode);

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useNodeAddForm(node);

  // const {
  //   isAddingDividerNode: isAddingDividerNodeAbove,
  //   setIsAddingDividerNode: setIsAddingDividerNodeAbove,
  //   handleAddDividerFormSubmit: handleAddDividerFormSubmitAbove,
  //   handleAddDividerFormCancel: handleAddDividerFormCancelAbove,
  // } = useAddNodeWithDivider(parentNode, onNodeUpdate, node.order - 1);

  // const {
  //   isAddingDividerNode: isAddingDividerNodeBelow,
  //   setIsAddingDividerNode: setIsAddingDividerNodeBelow,
  //   handleAddDividerFormSubmit: handleAddDividerFormSubmitBelow,
  //   handleAddDividerFormCancel: handleAddDividerFormCancelBelow,
  // } = useAddNodeWithDivider(parentNode, onNodeUpdate, node.order + 1);

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } =
    useNodeRenamingForm(node);

  const [preview, setPreview] = useState<HTMLElement | null>(null);

  const { instruction, isDragging, canDrop } = useInstructionNode(
    node,
    parentNode,
    id,
    triggerRef,
    isLastChild,
    isRootNode,
    setPreview
  );

  const handleDeleteNode = async () => {
    await deleteAndUpdatePeers();
  };

  const shouldRenderChildNodes = node.expanded || isAddingFileNode || isAddingFolderNode;
  const shouldRenderAddingFormDivider = false; // !isAddingDividerNodeAbove && !isAddingDividerNodeBelow;
  const nodePaddingLeft = depth * nodeOffset;
  const restrictedNames = parentNode?.childNodes.map((childNode) => childNode.name) ?? [];

  return (
    <li
      className={cn("relative", {
        // "background-(--moss-error-background)": instruction !== null && canDrop === false,
      })}
    >
      {isRenamingNode && !isRootNode ? (
        <TreeNodeRenameForm
          node={node}
          depth={depth}
          restrictedNames={restrictedNames}
          handleRenamingFormSubmit={handleRenamingFormSubmit}
          handleRenamingFormCancel={handleRenamingFormCancel}
        />
      ) : (
        <>
          {/* {shouldRenderAddingFormDivider && (
            <AddingDividerTrigger
              paddingLeft={nodePaddingLeft}
              paddingRight={paddingRight}
              position="top"
              onClick={() => setIsAddingDividerNodeAbove(true)}
            />
          )} */}

          {/* {isAddingDividerNodeAbove && (
            <TreeNodeAddForm
              depth={depth - 1}
              isAddingFolderNode={false}
              restrictedNames={restrictedNames}
              handleAddFormSubmit={handleAddDividerFormSubmitAbove}
              handleAddFormCancel={handleAddDividerFormCancelAbove}
            />
          )} */}

          {node.kind === "Dir" && instruction !== null && (
            <DropIndicatorWithInstruction
              paddingLeft={nodePaddingLeft}
              paddingRight={paddingRight}
              instruction={instruction}
              isFolder={true}
              depth={depth}
              isLastChild={isLastChild}
              canDrop={canDrop}
              gap={-2}
            />
          )}

          <TreeNodeButton
            ref={triggerRef}
            node={node}
            depth={depth}
            onAddFile={() => setIsAddingFileNode(true)}
            onAddFolder={() => setIsAddingFolderNode(true)}
            onRename={() => setIsRenamingNode(true)}
            onDelete={handleDeleteNode}
            isDragging={isDragging}
            canDrop={canDrop}
            instruction={instruction}
            preview={preview}
            isLastChild={isLastChild}
            isRootNode={isRootNode}
          />

          {/* 
          {isAddingDividerNodeBelow && (
            <TreeNodeAddForm
              node={node}
              depth={depth - 1}
              restrictedNames={restrictedNames}
              isAddingFolderNode={false}
              handleAddFormSubmit={handleAddDividerFormSubmitBelow}
              handleAddFormCancel={handleAddDividerFormCancelBelow}
            />
          )} */}

          {/* {shouldRenderAddingFormDivider && isLastChild && (
            <AddingDividerTrigger
              paddingLeft={nodePaddingLeft}
              paddingRight={paddingRight}
              position="bottom"
              onClick={() => setIsAddingDividerNodeBelow(true)}
            />
          )} */}
        </>
      )}

      {shouldRenderChildNodes && <TreeNodeChildren node={node} depth={depth} />}
      {(isAddingFileNode || isAddingFolderNode) && (
        <TreeNodeAddForm
          depth={depth}
          isAddingFolderNode={isAddingFolderNode}
          handleAddFormSubmit={handleAddFormSubmit}
          handleAddFormCancel={handleAddFormCancel}
          restrictedNames={restrictedNames}
        />
      )}
    </li>
  );
};

export default TreeNode;
