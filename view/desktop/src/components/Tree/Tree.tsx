import { createContext, useContext, useEffect, useId, useState } from "react";

import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import TreeNode from "./TreeNode";
import { NodeProps, TreeProps } from "./types";

interface TreeContextProps {
  dropSourceData: {
    node: NodeProps;
    TreeId: string;
  } | null;
  TreeId: string;
}

export const TreeContext = createContext<TreeContextProps>({
  dropSourceData: null,
  TreeId: "",
});

export const Tree = ({
  tree: initialTree,
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
  const TreeContextValues = useContext(TreeContext);

  const [dropSourceData, setDropSourceData] = useState<{
    node: NodeProps;
    TreeId: string;
  } | null>(null);

  const [tree, setTree] = useState<NodeProps>(initialTree);

  useEffect(() => {
    const handleMoveTreeNode = (event: CustomEvent) => {
      const { source, target } = event.detail as {
        source: {
          treeId: string;
          node: NodeProps;
        };
        target: {
          treeId: string;
          node: NodeProps;
        };
      };

      console.log("onDropTargetChange", source, target);

      if (source.treeId === target.treeId && source.node.id === target.node.id) return;
      if (source.node.id === target.node.id) return;

      if (source.treeId === TreeId) {
        const removeNode = (nodes: NodeProps[], nodeId: string | number): NodeProps[] => {
          return nodes
            .filter((n) => n.id !== nodeId)
            .map((n) => ({
              ...n,
              childNodes: n.childNodes ? removeNode(n.childNodes, nodeId) : [],
            }));
        };

        setTree((prev) => {
          return {
            ...prev,
            childNodes: removeNode(prev.childNodes, source.node.id),
          };
        });
      }

      if (target.treeId === TreeId) {
        const addNodeToTree = (tree: NodeProps, parentId: string | number, newNode: NodeProps): NodeProps => {
          if (tree.id === parentId) {
            return {
              ...tree,
              childNodes: [...(tree.childNodes || []), newNode],
            };
          } else if (tree.childNodes) {
            return {
              ...tree,
              childNodes: tree.childNodes.map((child) => addNodeToTree(child, parentId, newNode)),
            };
          }
          return tree;
        };

        setTree((prev) => addNodeToTree(prev, target.node.id, source.node));
      }
    };

    window.addEventListener("moveTreeNode", handleMoveTreeNode as EventListener);
    return () => {
      window.removeEventListener("moveTreeNode", handleMoveTreeNode as EventListener);
    };
  }, [TreeId]);

  useEffect(() => {
    return monitorForElements({
      onDropTargetChange: ({ location }) => {
        if (location.current?.dropTargets[0].data.depth === 0) {
          setDropSourceData({ node: tree, TreeId: location.current?.dropTargets[0].data.TreeId as string });
          return;
        }

        if (
          location.current?.dropTargets[0] &&
          (location.current.dropTargets[0].data as { node: NodeProps }).node?.isFolder
        ) {
          setDropSourceData(location.current.dropTargets[0].data as { node: NodeProps; TreeId: string });
        } else if (location.current?.dropTargets[1]) {
          setDropSourceData(location.current.dropTargets[1].data as { node: NodeProps; TreeId: string });
        } else if (location.current?.dropTargets[0]) {
          setDropSourceData(location.current.dropTargets[0].data as { node: NodeProps; TreeId: string });
        } else {
          setDropSourceData(null);
        }
      },
      onDrop: () => {
        setDropSourceData(null);
      },
    });
  }, [TreeContextValues.dropSourceData, TreeId, tree]);

  // const handleTreeUpdate = (updatedTree: NodeProps[]) => {
  //   setTree(updatedTree);
  //   onTreeUpdate?.(updatedTree);
  // };

  const hanldeOnNodeUpdate = (node: NodeProps) => {
    setTree(node);
    onNodeUpdate?.(node);
    onTreeUpdate?.(node);
  };

  return (
    <div className={className}>
      <TreeContext.Provider value={{ dropSourceData, TreeId }}>
        <TreeNode
          node={tree}
          onNodeUpdate={hanldeOnNodeUpdate}
          onNodeExpand={onNodeExpand}
          onNodeCollapse={onNodeCollapse}
          depth={0}
          horizontalPadding={horizontalPadding}
          nodeOffset={nodeOffset}
        />
      </TreeContext.Provider>
    </div>
  );
};
