import { projectService } from "@/domains/project/projectService";
import { BatchUpdateProjectInput, BatchUpdateProjectOutput, ListProjectItem, ListProjectsOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_PROJECTS_QUERY_KEY } from "./useListProjects";

export const useBatchUpdateProject = () => {
  const queryClient = useQueryClient();

  return useMutation<BatchUpdateProjectOutput, Error, BatchUpdateProjectInput>({
    mutationFn: (input) => projectService.batchUpdate(input),
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_LIST_PROJECTS_QUERY_KEY], (old: ListProjectsOutput | undefined) => {
        if (!old) return { items: [] };

        const newItems: ListProjectItem[] = old.items.map((oldProject): ListProjectItem => {
          const updatedProject = variables.items.find((project) => project.id === oldProject.id);
          if (updatedProject) {
            const iconPath =
              updatedProject.iconPath &&
              typeof updatedProject.iconPath === "object" &&
              "UPDATE" in updatedProject.iconPath
                ? updatedProject.iconPath.UPDATE
                : updatedProject.iconPath === "REMOVE"
                  ? undefined
                  : oldProject.iconPath;

            return {
              ...oldProject,
              ...updatedProject,
              iconPath,
            };
          }

          return oldProject;
        });

        return { items: newItems } satisfies ListProjectsOutput;
      });
    },
  });
};
