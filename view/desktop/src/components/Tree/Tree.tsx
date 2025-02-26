import { useState } from "react";

import TreeNode from "./TreeNode";

export interface ITreeNode {
  id: string | number;
  name: string;
  order: number;
  type: string;
  isExpanded: boolean;
  isFolder: boolean;
  childNodes: ITreeNode[];
}

interface TreeProps {
  nodes: ITreeNode[];
  onNodeUpdate?: (item: ITreeNode) => void;
  onChildNodesUpdate?: (items: ITreeNode[]) => void;
  onNodeExpand?: (node: ITreeNode) => void;
  onNodeCollapse?: (node: ITreeNode) => void;
  depth?: number;
}

export const Tree = ({
  nodes,
  onNodeUpdate,
  onChildNodesUpdate,
  onNodeExpand,
  onNodeCollapse,
  depth = 0,
}: TreeProps) => {
  const [treeNodes, setTreeNodes] = useState<ITreeNode[]>(nodes);

  const handleNodeUpdate = (updatedNode: ITreeNode) => {
    console.log({ updatedNode });
    const newTreeItems = treeNodes.map((node) => (node.id === updatedNode.id ? updatedNode : node));
    setTreeNodes(newTreeItems);
    if (onChildNodesUpdate) onChildNodesUpdate(newTreeItems);
    if (onNodeUpdate) onNodeUpdate(updatedNode);
  };

  return (
    <ul className="pl-4">
      {treeNodes.map((node) => (
        <TreeNode
          key={node.id}
          node={node}
          onNodeUpdate={handleNodeUpdate}
          onNodeExpand={onNodeExpand}
          onNodeCollapse={onNodeCollapse}
          depth={depth}
        />
      ))}
    </ul>
  );
};

export default Tree;
