import { useState } from "react";

import { useStreamProjects, useUpdateProject } from "../adapters/tanstackQuery/project";

export const useRenameProjectForm = (projectId: string) => {
  const [isRenamingProject, setIsRenamingProject] = useState(false);
  const { data: streamedProject } = useStreamProjects();
  const project = streamedProject?.find((project) => project.id === projectId);

  const { mutateAsync: updateProject } = useUpdateProject();

  const handleRenamingProjectFormSubmit = async (newName: string) => {
    const trimmedNewName = newName.trim();

    try {
      if (trimmedNewName === project?.name) {
        return;
      }

      await updateProject({
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
