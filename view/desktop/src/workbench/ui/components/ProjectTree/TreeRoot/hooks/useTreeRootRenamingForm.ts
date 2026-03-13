import { useState } from "react";

import { projectService } from "@/domains/project/projectService";

import { ProjectTree } from "../../types";

export const useTreeRootRenamingForm = (node: ProjectTree) => {
  const [isRenamingTreeRoot, setIsRenamingTreeRoot] = useState(false);

  const handleRenamingTreeRootFormSubmit = async (name: string) => {
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
      setIsRenamingTreeRoot(false);
    }
  };

  const handleRenamingTreeRootFormCancel = () => {
    setIsRenamingTreeRoot(false);
  };

  return {
    isRenamingTreeRoot,
    setIsRenamingTreeRoot,
    handleRenamingTreeRootFormSubmit,
    handleRenamingTreeRootFormCancel,
  };
};
