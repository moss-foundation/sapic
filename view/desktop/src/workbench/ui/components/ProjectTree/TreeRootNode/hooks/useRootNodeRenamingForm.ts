import { useState } from "react";

import { useUpdateProject } from "@/adapters/tanstackQuery/project";

import { ProjectTree } from "../../types";

export const useRootNodeRenamingForm = (node: ProjectTree) => {
  const [isRenamingRootNode, setIsRenamingRootNode] = useState(false);

  const { mutateAsync: updateProject } = useUpdateProject();

  const handleRenamingRootNodeFormSubmit = async (name: string) => {
    const trimmedNewName = name.trim();

    try {
      if (trimmedNewName === node.name) {
        return;
      }

      await updateProject({
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
    setIsRenamingRootNode(false);
  };

  return {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  };
};
