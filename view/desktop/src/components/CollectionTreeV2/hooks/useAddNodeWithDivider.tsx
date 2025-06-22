import { useContext, useState } from "react";

import { TreeContext } from "../Tree";
import { NodeProps, TreeNodeProps } from "../types";
import { prepareCollectionForTree, updateNodeOrder } from "../utils";

export const useAddNodeWithDivider = (
  node: TreeNodeProps,
  onNodeUpdateCallback: (node: TreeNodeProps) => void,
  order: number
) => {
  const { sortBy } = useContext(TreeContext);

  const [isAddingDividerNode, setIsAddingDividerNode] = useState(false);

  const handleAddDividerFormSubmit = (newNode: NodeProps) => {
    onNodeUpdateCallback(
      updateNodeOrder({
        ...node,
        isExpanded: true,
        childNodes: [
          ...node.childNodes.slice(0, order),
          prepareCollectionForTree(newNode, sortBy, false),
          ...node.childNodes.slice(order),
        ],
      })
    );

    setIsAddingDividerNode(false);
  };

  const handleAddDividerFormCancel = () => {
    setIsAddingDividerNode(false);
  };

  return {
    isAddingDividerNode,
    setIsAddingDividerNode,
    handleAddDividerFormSubmit,
    handleAddDividerFormCancel,
  };
};
