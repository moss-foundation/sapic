import { useContext } from "react";

import { cn } from "@/utils";

import { NodeAddForm } from "../NodeAddForm";
import { TestCollectionIcon } from "../TestCollectionIcon";
import { TreeContext } from "../Tree";
import TreeNode from "../TreeNode/TreeNode";
import { NodeProps, TreeNodeProps } from "../types";
import { hasDescendantWithSearchInput } from "../utils";

interface TreeRootNodeChildrenProps {
  node: TreeNodeProps;
  onNodeUpdate: (node: TreeNodeProps) => void;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleAddFormRootSubmit: (newNode: NodeProps) => void;
  handleAddFormRootCancel: () => void;
}

export const TreeRootNodeChildren = ({
  node,
  onNodeUpdate,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleAddFormRootSubmit,
  handleAddFormRootCancel,
}: TreeRootNodeChildrenProps) => {
  const { searchInput, nodeOffset, onRootAddCallback } = useContext(TreeContext);

  const filteredChildNodes = searchInput
    ? node.childNodes.filter((childNode) => hasDescendantWithSearchInput(childNode, searchInput))
    : node.childNodes;

  return (
    <ul className={cn("h-full w-full", { "pb-2": node.childNodes.length > 0 && node.isExpanded })}>
      {filteredChildNodes.map((childNode, index) => (
        <TreeNode
          parentNode={node}
          onNodeUpdate={onNodeUpdate}
          key={childNode.uniqueId}
          node={childNode}
          depth={1}
          isLastChild={index === filteredChildNodes.length - 1}
        />
      ))}
      {(isAddingRootFileNode || isAddingRootFolderNode) && (
        <div className="flex w-full min-w-0 items-center gap-1 py-0.5" style={{ paddingLeft: nodeOffset * 1 }}>
          <TestCollectionIcon type={node.type} className="opacity-0" />
          <TestCollectionIcon type={node.type} className={cn({ "opacity-0": isAddingRootFileNode })} />
          <NodeAddForm
            isFolder={isAddingRootFolderNode}
            restrictedNames={node.childNodes.map((childNode) => childNode.id)}
            onSubmit={(newNode) => {
              handleAddFormRootSubmit(newNode);
              onRootAddCallback?.({ ...node, childNodes: [...node.childNodes, newNode] } as TreeNodeProps);
            }}
            onCancel={handleAddFormRootCancel}
          />
        </div>
      )}
    </ul>
  );
};
