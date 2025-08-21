import { useState } from "react";

import { useUpdateCollection } from "@/hooks";

import { TreeCollectionRootNode } from "../../types";

export const useRootNodeRenamingForm = (node: TreeCollectionRootNode) => {
  const [isRenamingRootNode, setIsRenamingRootNode] = useState(false);

  const { mutateAsync: updateCollection } = useUpdateCollection();

  const handleRenamingRootNodeFormSubmit = async (name: string) => {
    try {
      if (name === node.name) {
        return;
      }

      await updateCollection({
        id: node.id,
        name,
      });
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingRootNode(false);
    }
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
