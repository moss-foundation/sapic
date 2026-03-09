import { useState } from "react";

import { projectService } from "@/domains/project/projectService";

import { ProjectTree } from "../../types";

export const useRootNodeRenamingForm = (node: ProjectTree) => {
  const [isRenamingRootNode, setIsRenamingRootNode] = useState(false);

  const handleRenamingRootNodeFormSubmit = async (name: string) => {
    const trimmedNewName = name.trim();

    try {
      if (trimmedNewName === node.name) {
        return;
      }

      await projectService.update({
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
