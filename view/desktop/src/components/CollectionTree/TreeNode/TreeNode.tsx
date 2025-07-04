import { useContext } from "react";

import { useDeleteCollectionEntry } from "@/hooks";

import { TreeContext } from "../..";
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

export interface TreeNodeComponentProps extends NodeEvents {
  node: TreeCollectionNode;
  depth: number;
  parentNode?: TreeCollectionNode;
  isLastChild: boolean;
}

export interface NodeEvents {
  onNodeUpdate: (node: TreeCollectionNode) => void;
}

export const TreeNode = ({ node, onNodeUpdate, depth, parentNode, isLastChild }: TreeNodeComponentProps) => {
  const { nodeOffset, paddingRight, id } = useContext(TreeContext);
  const { mutateAsync: deleteCollectionEntry } = useDeleteCollectionEntry();
  // const triggerRef = useRef<HTMLButtonElement>(null);

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useNodeAddForm(node, onNodeUpdate);

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

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useNodeRenamingForm(
    node,
    onNodeUpdate
  );

  const handleDeleteNode = () => {
    deleteCollectionEntry({
      collectionId: id,
      input: {
        id: node.id,
      },
    });
    // onNodeUpdate(node);
  };
  // const [preview, setPreview] = useState<HTMLElement | null>(null);
  // const { instruction, isDragging, canDrop } = useInstructionNode(node, treeId, triggerRef, isLastChild, setPreview);

  const shouldRenderChildNodes = node.expanded || isAddingFileNode || isAddingFolderNode;
  const shouldRenderAddingFormDivider = false; // !isAddingDividerNodeAbove && !isAddingDividerNodeBelow;
  const nodePaddingLeft = depth * nodeOffset;
  const restrictedNames = parentNode?.childNodes.map((childNode) => childNode.name) ?? [];
  const isRootNode = node.path.segments.length === 1;

  return (
    <li className="relative">
      {/* {node.isFolder && instruction !== null && canDrop === true && (
        <DropIndicatorWithInstruction
          paddingLeft={nodePaddingLeft}
          paddingRight={paddingRight}
          instruction={instruction}
          isFolder={node.isFolder}
          depth={depth}
          isLastChild={isLastChild}
        />
      )} */}
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

          <TreeNodeButton
            // ref={triggerRef}
            node={node}
            onNodeUpdate={onNodeUpdate}
            depth={depth}
            onAddFile={() => setIsAddingFileNode(true)}
            onAddFolder={() => setIsAddingFolderNode(true)}
            onRename={() => setIsRenamingNode(true)}
            onDelete={handleDeleteNode}
            // isDragging={isDragging}
            // canDrop={canDrop}
            // instruction={instruction}
            // preview={preview}
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
      {shouldRenderChildNodes && <TreeNodeChildren node={node} onNodeUpdate={onNodeUpdate} depth={depth} />}
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
