import { createContext, useEffect, useId, useState } from "react";

import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import RecursiveTree from "./RecursiveTree";
import { NodeProps, TreeProps } from "./types";

interface TreeContextProps {
  dropTargetData: {
    node: NodeProps;
    TreeId: string;
  } | null;
  TreeId: string;
}

export const TreeContext = createContext<TreeContextProps>({
  dropTargetData: null,
  TreeId: "",
});

export const Tree = ({
  nodes,
  onNodeUpdate,
  onNodeExpand,
  onNodeCollapse,
  onTreeUpdate,
  horizontalPadding = 16,
  nodeOffset = 16,
  sortBy = "none",
  className,
}: TreeProps) => {
  const TreeId = useId();

  const [dropTargetData, setDropTargetData] = useState<{
    node: NodeProps;
    TreeId: string;
  } | null>(null);

  useEffect(() => {
    return monitorForElements({
      onDropTargetChange: (args) => {
        if (args.location.current?.dropTargets[0] && args.location.current?.dropTargets[0].data.node.isFolder) {
          setDropTargetData(args.location.current.dropTargets[0].data as { node: NodeProps; TreeId: string });
        } else if (args.location.current?.dropTargets[1]) {
          setDropTargetData(args.location.current.dropTargets[1].data as { node: NodeProps; TreeId: string });
        } else if (args.location.current?.dropTargets[0]) {
          setDropTargetData(args.location.current.dropTargets[0].data as { node: NodeProps; TreeId: string });
        } else {
          setDropTargetData(null);
        }
      },
      onDrop: () => {
        setDropTargetData(null);
      },
    });
  }, []);

  const [treeNodes, setTreeNodes] = useState<NodeProps[]>(nodes);

  const handleTreeUpdate = (updatedTree: NodeProps[]) => {
    setTreeNodes(updatedTree);
    onTreeUpdate?.(updatedTree);
  };

  return (
    <div className={className}>
      <TreeContext.Provider value={{ dropTargetData, TreeId }}>
        <RecursiveTree
          nodes={treeNodes}
          onNodeUpdate={onNodeUpdate}
          onNodeExpand={onNodeExpand}
          onNodeCollapse={onNodeCollapse}
          onTreeUpdate={handleTreeUpdate}
          horizontalPadding={horizontalPadding}
          nodeOffset={nodeOffset}
        />
      </TreeContext.Provider>
    </div>
  );
};
