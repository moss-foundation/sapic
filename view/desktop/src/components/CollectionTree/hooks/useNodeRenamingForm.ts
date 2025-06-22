import { useState } from "react";

import { TreeCollectionNode } from "../types";

export const useNodeRenamingForm = (node: TreeCollectionNode, onNodeUpdate: (node: TreeCollectionNode) => void) => {
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const handleRenamingFormSubmit = (newName: string) => {
    onNodeUpdate({ ...node, name: newName });

    setIsRenamingNode(false);
  };

  const handleRenamingFormCancel = () => {
    setIsRenamingNode(false);
  };

  return {
    isRenamingNode,
    setIsRenamingNode,
    handleRenamingFormSubmit,
    handleRenamingFormCancel,
  };
};
