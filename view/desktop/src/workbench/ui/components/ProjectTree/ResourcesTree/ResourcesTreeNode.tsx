import { useContext, useRef, useState } from "react";

import { Tree } from "@/lib/ui/Tree";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { IResourcesTree, ResourceNode } from "../types";
import { getChildrenNames } from "../utils";
import { useDraggableResourceNode } from "./dnd/hooks/useDraggableResourceNode";
import { useMonitorDirForBlockedChildOperation } from "./dnd/hooks/useMonitorDirForBlockedChildOperation";
import ResourceNodeAddForm from "./forms/ResourceNodeAddForm";
import ResourceNodeRenamingForm from "./forms/ResourceNodeRenamingForm";
import { useDeleteAndUpdateResourceNodePeers } from "./hooks/useDeleteAndUpdateResourceNodePeers";
import { useResourceNodeAddForm } from "./hooks/useResourceNodeAddForm";
import { useResourceNodeRenamingForm } from "./hooks/useResourceNodeRenamingForm";
import { ResourcesTreeChildren } from "./ResourcesTreeChildren";
import ResourcesTreeNodeDetails from "./ResourcesTreeNodeDetails";

interface ResourcesTreeNodeProps {
  node: ResourceNode;
  parentNode: ResourceNode | IResourcesTree;
  depth: number;
}

export const ResourcesTreeNode = ({ node, parentNode, depth }: ResourcesTreeNodeProps) => {
  const { id } = useContext(ProjectTreeContext);

  const triggerRef = useRef<HTMLDivElement>(null);
  const nodeRef = useRef<HTMLLIElement>(null);

  const { deleteAndUpdatePeers } = useDeleteAndUpdateResourceNodePeers({ projectId: id, node, parentNode });

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useResourceNodeAddForm(node);

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } =
    useResourceNodeRenamingForm({ node, projectId: id });

  const handleDeleteNode = async () => {
    await deleteAndUpdatePeers();
  };

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const { instruction, isDragging } = useDraggableResourceNode({
    node,
    parentNode,
    triggerRef,
    setPreview,
  });

  const { childNodeHasBlockedOperation } = useMonitorDirForBlockedChildOperation({
    nodeRef,
    node,
    parentNode,
  });

  const shouldRenderChildNodes = node.expanded || isAddingFileNode || isAddingFolderNode;
  const restrictedNames = getChildrenNames(node);

  return (
    <Tree.Node
      ref={nodeRef}
      combineInstruction={instruction}
      isDragging={isDragging}
      childNodeHasBlockedOperation={childNodeHasBlockedOperation}
    >
      {isRenamingNode ? (
        <ResourceNodeRenamingForm
          node={node}
          depth={depth}
          restrictedNames={restrictedNames}
          handleRenamingFormSubmit={handleRenamingFormSubmit}
          handleRenamingFormCancel={handleRenamingFormCancel}
        />
      ) : (
        <ResourcesTreeNodeDetails
          ref={triggerRef}
          node={node}
          parentNode={parentNode}
          depth={depth}
          onAddFile={() => setIsAddingFileNode(true)}
          onAddFolder={() => setIsAddingFolderNode(true)}
          onRename={() => setIsRenamingNode(true)}
          onDelete={handleDeleteNode}
          isDragging={isDragging}
          reorderInstruction={instruction}
          preview={preview}
        />
      )}

      {shouldRenderChildNodes && (
        <ResourcesTreeChildren rootResourcesNodes={node.childNodes} parentNode={node} depth={depth + 1} />
      )}

      {(isAddingFileNode || isAddingFolderNode) && (
        <ResourceNodeAddForm
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
