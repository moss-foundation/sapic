import { useState } from "react";

import { TreeCollectionNode } from "../types";

interface UseAddNodeWithDividerProps {
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
  position: "above" | "below";
}

export const useAddNodeWithDivider = ({}: UseAddNodeWithDividerProps) => {
  const [isAddingDividerNode, setIsAddingDividerNode] = useState(false);

  const handleAddDividerFormSubmit = () => {
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
