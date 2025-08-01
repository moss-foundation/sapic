import { useContext, useRef, useState } from "react";

import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils/cn";

import { TreeContext } from "../..";
import { useDeleteAndUpdatePeers } from "../actions/useDeleteAndUpdatePeers";
import { ActiveNodeIndicator } from "../ActiveNodeIndicator";
import { DropIndicatorWithInstruction } from "../DropIndicatorWithInstruction";
import { useDraggableNode } from "../hooks/useDraggableNode";
import { useNodeAddForm } from "../hooks/useNodeAddForm";
import { useNodeRenamingForm } from "../hooks/useNodeRenamingForm";
import { TreeCollectionNode } from "../types";
import TreeNodeAddForm from "./TreeNodeAddForm";
import TreeNodeButton from "./TreeNodeButton";
import TreeNodeChildren from "./TreeNodeChildren";
import TreeNodeRenameForm from "./TreeNodeRenameForm";

export interface TreeNodeComponentProps {
  node: TreeCollectionNode;
  depth: number;
  parentNode: TreeCollectionNode;
  isLastChild: boolean;
  isRootNode?: boolean;
}

export const TreeNode = ({ node, depth, parentNode, isLastChild, isRootNode = false }: TreeNodeComponentProps) => {
  const { nodeOffset, treePaddingRight, id } = useContext(TreeContext);

  const triggerRef = useRef<HTMLButtonElement>(null);

  const { deleteAndUpdatePeers } = useDeleteAndUpdatePeers(id, node, parentNode);

  const { activePanelId } = useTabbedPaneStore();

  // const {
  //   isAddingDividerNode: isAddingDividerNodeAbove,
  //   setIsAddingDividerNode: setIsAddingDividerNodeAbove,
  //   handleAddDividerFormSubmit: handleAddDividerFormSubmitAbove,
  //   handleAddDividerFormCancel: handleAddDividerFormCancelAbove,
  // } = useAddNodeWithDivider({ node, parentNode, position: "above" });

  // const {
  //   isAddingDividerNode: isAddingDividerNodeBelow,
  //   setIsAddingDividerNode: setIsAddingDividerNodeBelow,
  //   handleAddDividerFormSubmit: handleAddDividerFormSubmitBelow,
  //   handleAddDividerFormCancel: handleAddDividerFormCancelBelow,
  // } = useAddNodeWithDivider({ node, parentNode, position: "below" });

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useNodeAddForm(node);

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } =
    useNodeRenamingForm(node);

  const handleDeleteNode = async () => {
    await deleteAndUpdatePeers();
  };

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const { instruction, isDragging, canDrop } = useDraggableNode(
    node,
    parentNode,
    id,
    triggerRef,
    isLastChild,
    isRootNode,
    setPreview
  );

  const shouldRenderChildNodes = node.expanded || isAddingFileNode || isAddingFolderNode;
  const shouldRenderAddingFormDivider = false; // !isAddingDividerNodeAbove && !isAddingDividerNodeBelow;
  const nodePaddingLeft = depth * nodeOffset;
  const restrictedNames = parentNode?.childNodes.map((childNode) => childNode.name) ?? [];

  return (
    <li className="relative">
      {isRenamingNode && !isRootNode ? (
        <TreeNodeRenameForm
          node={node}
          depth={depth}
          restrictedNames={restrictedNames}
          handleRenamingFormSubmit={handleRenamingFormSubmit}
          handleRenamingFormCancel={handleRenamingFormCancel}
        />
      ) : (
        <div
          className={cn(
            "hover:background-(--moss-secondary-background-hover) relative flex items-center justify-between",
            {
              "background-(--moss-secondary-background-hover)": activePanelId === node.id,
            }
          )}
        >
          {activePanelId === node.id && <ActiveNodeIndicator />}
          {/* {shouldRenderAddingFormDivider && (
            <AddingDividerTrigger
              paddingLeft={nodePaddingLeft}
              paddingRight={paddingRight}
              position="above"
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
              paddingRight={treePaddingRight}
              instruction={instruction}
              isFolder={true}
              depth={depth}
              isLastChild={isLastChild}
              canDrop={canDrop}
              gap={-1}
            />
          )}

          <TreeNodeButton
            ref={triggerRef}
            node={node}
            parentNode={parentNode}
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
              position="below"
              onClick={() => setIsAddingDividerNodeBelow(true)}
            />
          )} */}
        </div>
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
