/* eslint-disable */
import { useContext, useState } from "react";

import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";

export const useAddNodeWithDivider = (
  node: TreeCollectionNode,
  onNodeUpdateCallback: (node: TreeCollectionNode) => void,
  order: number
) => {
  const { sortBy } = useContext(TreeContext);

  const [isAddingDividerNode, setIsAddingDividerNode] = useState(false);

  const handleAddDividerFormSubmit = (newNode: NodeProps) => {
    onNodeUpdateCallback(
      updateNodeOrder({
        ...node,
        expanded: true,
        childNodes: [...node.childNodes.slice(0, order), newNode, ...node.childNodes.slice(order)],
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
