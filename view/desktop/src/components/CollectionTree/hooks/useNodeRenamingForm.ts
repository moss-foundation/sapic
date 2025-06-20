import { useState } from "react";

import { TreeNodeProps } from "../types";

export const useNodeRenamingForm = (node: TreeNodeProps, onNodeUpdate: (node: TreeNodeProps) => void) => {
  const [isRenamingNode, setIsRenamingNode] = useState(false);

  const handleRenamingFormSubmit = (newId: string) => {
    onNodeUpdate({ ...node, id: newId });

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
