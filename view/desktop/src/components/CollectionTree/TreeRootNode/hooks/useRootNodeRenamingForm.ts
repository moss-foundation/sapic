import { useState } from "react";

import { useUpdateCollection } from "@/hooks";

import { TreeCollectionRootNode } from "../../types";

export const useRootNodeRenamingForm = (node: TreeCollectionRootNode) => {
  const [isRenamingRootNode, setIsRenamingRootNode] = useState(false);

  const { mutateAsync: updateCollection } = useUpdateCollection();

  const handleRenamingRootNodeFormSubmit = (name: string) => {
    updateCollection({
      id: node.id,
      name,
    });

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
