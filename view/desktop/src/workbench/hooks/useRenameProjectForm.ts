import { useState } from "react";

import { useGetLocalProjectSummaryById } from "@/db/projectSummaries/hooks/useGetLocalProjectSummaryById";
import { projectService } from "@/domains/project/projectService";

export const useRenameProjectForm = (projectId: string) => {
  const [isRenamingProject, setIsRenamingProject] = useState(false);
  const projectSummary = useGetLocalProjectSummaryById(projectId);

  const handleRenamingProjectFormSubmit = async (newName: string) => {
    const trimmedNewName = newName.trim();

    try {
      if (trimmedNewName === projectSummary?.name) {
        return;
      }

      await projectService.updateProject({
        id: projectId,
        name: trimmedNewName,
      });
    } catch (error) {
      console.error(error);
    } finally {
      setIsRenamingProject(false);
    }
  };

  const handleRenamingProjectFormCancel = () => {
    setIsRenamingProject(false);
  };

  return {
    isRenamingProject,
    setIsRenamingProject,
    handleRenamingProjectFormSubmit,
    handleRenamingProjectFormCancel,
  };
};
