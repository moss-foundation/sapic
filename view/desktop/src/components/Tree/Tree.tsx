import RecursiveTree from "./RecursiveTree";
import { TreeProps } from "./types";

export const Tree = ({
  nodes,
  onNodeUpdate,
  onNodeExpand,
  onNodeCollapse,
  onTreeUpdate,
  horizontalPadding = 16,
  nodeOffset = 16,
}: TreeProps) => {
  return (
    <div>
      <RecursiveTree
        nodes={nodes}
        onNodeUpdate={onNodeUpdate}
        onNodeExpand={onNodeExpand}
        onNodeCollapse={onNodeCollapse}
        onTreeUpdate={onTreeUpdate}
        horizontalPadding={horizontalPadding}
        nodeOffset={nodeOffset}
      />
    </div>
  );
};
