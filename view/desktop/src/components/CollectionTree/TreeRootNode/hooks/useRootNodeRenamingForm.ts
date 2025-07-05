import { useState } from "react";

import { useUpdateCollection } from "@/hooks";

import { TreeCollectionRootNode } from "../../types";

export const useRootNodeRenamingForm = (
  node: TreeCollectionRootNode,
  onNodeUpdate: (node: TreeCollectionRootNode) => void
) => {
  const [isRenamingRootNode, setIsRenamingRootNode] = useState(false);

  const { placeholderFnForUpdateCollection } = useUpdateCollection();

  const handleRenamingRootNodeFormSubmit = (name: string) => {
    // onNodeUpdate?.({ ...node, name });

    placeholderFnForUpdateCollection({
      id: node.id,
      collection: {
        ...node,
        name,
      },
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
