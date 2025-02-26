import { useEffect, useState } from "react";

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

// export const TreeContext = createContext({});

interface TreeProps {
  nodes: ITreeNode[];
  onNodeUpdate?: (item: ITreeNode) => void;
  onChildNodesUpdate?: (items: ITreeNode[]) => void;
  depth?: number;
}

export const Tree = ({ nodes, onNodeUpdate, onChildNodesUpdate, depth = 0 }: TreeProps) => {
  const [treeNodes, setTreeNodes] = useState<ITreeNode[]>(nodes);

  const handleNodeUpdate = (updatedNode: ITreeNode) => {
    const newTreeItems = treeNodes.map((node) => (node.id === updatedNode.id ? updatedNode : node));

    setTreeNodes(newTreeItems);

    if (onChildNodesUpdate) onChildNodesUpdate(newTreeItems);
    if (onNodeUpdate) onNodeUpdate(updatedNode);
  };

  return (
    <ul className="pl-4">
      {treeNodes.map((node) => (
        <TreeNode key={node.id} node={node} onNodeUpdate={handleNodeUpdate} depth={depth} />
      ))}
    </ul>
  );
};

export default Tree;
