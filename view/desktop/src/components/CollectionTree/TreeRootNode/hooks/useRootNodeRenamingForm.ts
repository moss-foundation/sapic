import { useState } from "react";

import { TreeCollectionRootNode } from "../../types";

export const useRootNodeRenamingForm = (
  node: TreeCollectionRootNode,
  onNodeUpdate: (node: TreeCollectionRootNode) => void
) => {
  const [isRenamingRootNode, setIsRenamingRootNode] = useState(false);

  const handleRenamingRootNodeFormSubmit = (name: string) => {
    onNodeUpdate?.({ ...node, name });

    setIsRenamingRootNode(false);
  };

  const handleRenamingRootNodeFormCancel = () => {
    setIsRenamingRootNode(false);
  };

  return {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  };
};
