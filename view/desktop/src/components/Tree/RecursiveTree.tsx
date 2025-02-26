import { useState } from "react";

import TreeNode from "./TreeNode";
import { NodeProps, RecursiveTreeProps } from "./types";

export const RecursiveTree = ({
  nodes,
  onNodeUpdate,
  onChildNodesUpdate,
  onNodeExpand,
  onNodeCollapse,
  onTreeUpdate,
  depth = 0,
  horizontalPadding,
  nodeOffset,
}: RecursiveTreeProps) => {
  const [treeNodes, setTreeNodes] = useState<NodeProps[]>(nodes);

  const handleNodeUpdate = (updatedNode: NodeProps) => {
    const newTreeItems = treeNodes.map((node) => (node.id === updatedNode.id ? updatedNode : node));

    setTreeNodes(newTreeItems);

    onNodeUpdate?.(updatedNode);
    onChildNodesUpdate?.(newTreeItems);
    onTreeUpdate?.(newTreeItems);
  };

  return (
    <ul>
      {treeNodes.map((node) => (
        <TreeNode
          key={node.id}
          node={node}
          onNodeUpdate={handleNodeUpdate}
          onNodeExpand={onNodeExpand}
          onNodeCollapse={onNodeCollapse}
          depth={depth}
          horizontalPadding={horizontalPadding}
          nodeOffset={nodeOffset}
        />
      ))}
    </ul>
  );
};

export default RecursiveTree;
