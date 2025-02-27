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
  const handleNodeUpdate = (updatedNode: NodeProps) => {
    const newTreeItems = nodes.map((node) => (node.id === updatedNode.id ? updatedNode : node));

    onNodeUpdate?.(updatedNode);
    onChildNodesUpdate?.(newTreeItems);
    onTreeUpdate?.(newTreeItems);
  };

  return (
    <ul>
      {nodes.map((node) => (
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
