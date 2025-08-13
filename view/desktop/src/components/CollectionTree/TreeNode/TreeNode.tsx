import { useContext, useRef, useState } from "react";

import { cn } from "@/utils/cn";

import { useDeleteAndUpdatePeers } from "../actions/useDeleteAndUpdatePeers";
import { CollectionTreeContext } from "../CollectionTreeContext";
import { DropIndicatorForDir } from "../DropIndicatorForDir";
import { TreeCollectionNode } from "../types";
import { getChildrenNames } from "../utils";
import { useDraggableNode } from "./hooks/useDraggableNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
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
  const { id } = useContext(CollectionTreeContext);

  const triggerRef = useRef<HTMLDivElement>(null);
  const dropTargetListRef = useRef<HTMLLIElement>(null);

  const { deleteAndUpdatePeers } = useDeleteAndUpdatePeers(id, node, parentNode);

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
  const { instruction, isDragging, isChildDropBlocked } = useDraggableNode({
    node,
    parentNode,
    triggerRef,
    dropTargetListRef,
    isLastChild,
    isRootNode,
    setPreview,
  });

  const shouldRenderChildNodes = node.expanded || isAddingFileNode || isAddingFolderNode;
  const restrictedNames = getChildrenNames(node);

  return (
    <li ref={dropTargetListRef} className={cn("relative")}>
      <DropIndicatorForDir isChildDropBlocked={isChildDropBlocked} instruction={instruction} />

      {isRenamingNode && !isRootNode ? (
        <TreeNodeRenameForm
          node={node}
          depth={depth}
          restrictedNames={restrictedNames}
          handleRenamingFormSubmit={handleRenamingFormSubmit}
          handleRenamingFormCancel={handleRenamingFormCancel}
        />
      ) : (
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
          instruction={instruction}
          preview={preview}
          isLastChild={isLastChild}
          isRootNode={isRootNode}
          isChildDropBlocked={isChildDropBlocked}
        />
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
