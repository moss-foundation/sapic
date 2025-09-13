import { useState } from "react";

import { useUpdateCollection } from "@/hooks";

import { ProjectTreeRootNode } from "../../types";

export const useRootNodeRenamingForm = (node: ProjectTreeRootNode) => {
  const [isRenamingRootNode, setIsRenamingRootNode] = useState(false);

  const { mutateAsync: updateCollection } = useUpdateCollection();

  const handleRenamingRootNodeFormSubmit = async (name: string) => {
    const trimmedNewName = name.trim();

    try {
      if (trimmedNewName === node.name) {
        return;
      }

      await updateCollection({
        id: node.id,
        name: trimmedNewName,
      });
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingRootNode(false);
    }
  };

  const handleRenamingRootNodeFormCancel = () => {
    console.log("handleRenamingRootNodeFormCancel");
    setIsRenamingRootNode(false);
  };

  return {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  };
};
