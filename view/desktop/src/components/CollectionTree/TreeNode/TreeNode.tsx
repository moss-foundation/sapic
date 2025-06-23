import { useContext } from "react";

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
  const { onNodeAddCallback, onNodeRenameCallback, nodeOffset, paddingRight } = useContext(TreeContext);

  // const triggerRef = useRef<HTMLButtonElement>(null);

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

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useNodeRenamingForm(
    node,
    onNodeUpdate
  );

  // const [preview, setPreview] = useState<HTMLElement | null>(null);
  // const { instruction, isDragging, canDrop } = useInstructionNode(node, treeId, triggerRef, isLastChild, setPreview);

  const shouldRenderChildNodes = node.expanded || isAddingFileNode || isAddingFolderNode; //shouldRenderTreeNode(node, searchInput, isAddingFileNode, isAddingFolderNode);
  const nodePaddingLeft = depth * nodeOffset;

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
      {isRenamingNode ? (
        <TreeNodeRenameForm
          node={node}
          depth={depth}
          restrictedNames={parentNode?.childNodes.map((childNode) => childNode.id) ?? []}
          onNodeRenameCallback={onNodeRenameCallback}
          handleRenamingFormSubmit={handleRenamingFormSubmit}
          handleRenamingFormCancel={handleRenamingFormCancel}
        />
      ) : (
        <>
          {/* <AddingFormDivider
            paddingLeft={nodePaddingLeft}
            paddingRight={paddingRight}
            position="top"
            onClick={() => setIsAddingDividerNodeAbove(true)}
          /> */}

          {/* {isAddingDividerNodeAbove && (
            <TreeNodeAddForm
              node={node}
              depth={depth - 1}
              isAddingFileNode={true}
              isAddingFolderNode={false}
              onNodeAddCallback={onNodeAddCallback}
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
            // isDragging={isDragging}
            // canDrop={canDrop}
            // instruction={instruction}
            // preview={preview}
            isLastChild={isLastChild}
          />

          {/* {isAddingDividerNodeBelow && (
            <TreeNodeAddForm
              node={node}
              depth={depth - 1}
              isAddingFileNode={true}
              isAddingFolderNode={false}
              onNodeAddCallback={onNodeAddCallback}
              handleAddFormSubmit={handleAddDividerFormSubmitBelow}
              handleAddFormCancel={handleAddDividerFormCancelBelow}
            />
          )} */}

          {/* {isLastChild && (
            <AddingFormDivider
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
          node={node}
          depth={depth}
          isAddingFileNode={isAddingFileNode}
          isAddingFolderNode={isAddingFolderNode}
          onNodeAddCallback={onNodeAddCallback}
          handleAddFormSubmit={handleAddFormSubmit}
          handleAddFormCancel={handleAddFormCancel}
        />
      )}
    </li>
  );
};

export default TreeNode;
