import { useContext } from "react";

import { useDeleteCollectionEntry } from "@/hooks";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";

import { TreeContext } from "../..";
import { AddingDividerTrigger } from "../AddingFormDivider";
import { useAddNodeWithDivider } from "../hooks/useAddNodeWithDivider";
import { useNodeAddForm } from "../hooks/useNodeAddForm";
import { useNodeRenamingForm } from "../hooks/useNodeRenamingForm";
import { TreeCollectionNode, TreeNodeComponentProps } from "../types";
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

export const TreeNode = ({
  node,
  onNodeUpdate,
  depth,
  parentNode,
  isLastChild,
  isRootNode = false,
}: TreeNodeComponentProps) => {
  const { nodeOffset, paddingRight, id } = useContext(TreeContext);
  const { mutateAsync: deleteCollectionEntry } = useDeleteCollectionEntry();
  const { mutateAsync: batchUpdateCollectionEntries } = useBatchUpdateCollectionEntry();
  // const triggerRef = useRef<HTMLButtonElement>(null);

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
  } = useAddNodeWithDivider(node, parentNode, "before");

  const {
    isAddingDividerNode: isAddingDividerNodeBelow,
    setIsAddingDividerNode: setIsAddingDividerNodeBelow,
    handleAddDividerFormSubmit: handleAddDividerFormSubmitBelow,
    handleAddDividerFormCancel: handleAddDividerFormCancelBelow,
  } = useAddNodeWithDivider(node, parentNode, "after");

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useNodeRenamingForm(
    node,
    onNodeUpdate
  );

  const handleDeleteNode = async () => {
    try {
      const result = await deleteCollectionEntry({
        collectionId: id,
        input: {
          id: node.id,
        },
      });

      const nodesToUpdate = parentNode.childNodes
        .filter((childNode) => childNode.id !== result.id)
        .sort((a, b) => a.order! - b.order!)
        .map((childNode, index) => ({
          id: childNode.id,
          path: childNode.path.raw,
          order: index + 1,
          kind: childNode.kind,
        }));

      await batchUpdateCollectionEntries({
        collectionId: id,
        entries: {
          entries: nodesToUpdate.map((node) => {
            if (node.kind === "Dir") {
              return {
                DIR: {
                  id: node.id,
                  order: node.order,
                  path: node.path,
                },
              };
            }

            return {
              ITEM: {
                id: node.id,
                order: node.order,
                path: node.path,
              },
            };
          }),
        },
      });
    } catch (error) {
      console.error(error);
    }
  };
  // const [preview, setPreview] = useState<HTMLElement | null>(null);
  // const { instruction, isDragging, canDrop } = useInstructionNode(node, treeId, triggerRef, isLastChild, setPreview);

  const shouldRenderChildNodes = node.expanded || isAddingFileNode || isAddingFolderNode;
  const shouldRenderAddingFormDivider = !isAddingDividerNodeAbove && !isAddingDividerNodeBelow;
  const nodePaddingLeft = depth * nodeOffset;
  const restrictedNames =
    parentNode && "childNodes" in parentNode ? parentNode.childNodes.map((childNode) => childNode.name) : [];

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
          {shouldRenderAddingFormDivider && (
            <AddingDividerTrigger
              paddingLeft={nodePaddingLeft}
              paddingRight={paddingRight}
              position="top"
              onClick={() => setIsAddingDividerNodeAbove(true)}
            />
          )}

          {isAddingDividerNodeAbove && (
            <TreeNodeAddForm
              depth={depth - 1}
              isAddingFolderNode={false}
              restrictedNames={restrictedNames}
              handleAddFormSubmit={handleAddDividerFormSubmitAbove}
              handleAddFormCancel={handleAddDividerFormCancelAbove}
            />
          )}

          <TreeNodeButton
            // ref={triggerRef}
            node={node}
            onNodeUpdate={onNodeUpdate}
            depth={depth}
            onAddFile={() => setIsAddingFileNode(true)}
            onAddFolder={() => setIsAddingFolderNode(true)}
            onRename={() => setIsRenamingNode(true)}
            onDelete={handleDeleteNode}
            isDragging={false}
            canDrop={null}
            instruction={null}
            preview={null}
            isLastChild={isLastChild}
            isRootNode={isRootNode}
          />

          {isAddingDividerNodeBelow && (
            <TreeNodeAddForm
              depth={depth - 1}
              restrictedNames={restrictedNames}
              isAddingFolderNode={false}
              handleAddFormSubmit={handleAddDividerFormSubmitBelow}
              handleAddFormCancel={handleAddDividerFormCancelBelow}
            />
          )}

          {shouldRenderAddingFormDivider && isLastChild && (
            <AddingDividerTrigger
              paddingLeft={nodePaddingLeft}
              paddingRight={paddingRight}
              position="bottom"
              onClick={() => setIsAddingDividerNodeBelow(true)}
            />
          )}
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
