import { useContext, useRef, useState } from "react";

import { Tree } from "@/lib/ui/Tree";

import { useDeleteAndUpdatePeers } from "../actions/useDeleteAndUpdatePeers";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../types";
import { getChildrenNames } from "../utils";
import { useDraggableNode } from "./hooks/useDraggableNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
import TreeNodeAddForm from "./TreeNodeAddForm";
import TreeNodeChildren from "./TreeNodeChildren";
import TreeNodeControls from "./TreeNodeControls";
import TreeNodeRenamingForm from "./TreeNodeRenamingForm";

export interface TreeNodeComponentProps {
  node: ProjectTreeNode;
  depth: number;
  parentNode: ProjectTreeNode | ProjectTreeRootNode;
  isLastChild: boolean;
}

export const TreeNode = ({ node, depth, parentNode, isLastChild }: TreeNodeComponentProps) => {
  const { id } = useContext(ProjectTreeContext);

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
    setPreview,
  });

  const shouldRenderChildNodes = node.expanded || isAddingFileNode || isAddingFolderNode;
  const restrictedNames = getChildrenNames(node);

  return (
    <Tree.Node ref={dropTargetListRef} isChildDropBlocked={isChildDropBlocked} dropInstructionForDir={instruction}>
      {isRenamingNode ? (
        <TreeNodeRenamingForm
          node={node}
          depth={depth}
          restrictedNames={restrictedNames}
          handleRenamingFormSubmit={handleRenamingFormSubmit}
          handleRenamingFormCancel={handleRenamingFormCancel}
        />
      ) : (
        <TreeNodeControls
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
          isLastChild={isLastChild}
          preview={preview}
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
    </Tree.Node>
  );
};

export default TreeNode;
