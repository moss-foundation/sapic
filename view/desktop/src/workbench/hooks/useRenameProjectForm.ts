import { useState } from "react";

import { useUpdateProject } from "@/adapters/tanstackQuery/project";
import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";

export const useRenameProjectForm = (projectId: string) => {
  const [isRenamingProject, setIsRenamingProject] = useState(false);
  const { data: projects } = useListProjects();
  const project = projects?.items.find((project) => project.id === projectId);

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
