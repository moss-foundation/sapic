import { projectService } from "@/domains/project/projectService";
import { ListProjectItem, ListProjectsOutput, UpdateProjectInput, UpdateProjectOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_PROJECTS_QUERY_KEY } from "./useListProjects";

export const useUpdateProject = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateProjectOutput, Error, UpdateProjectInput>({
    mutationFn: (input) => projectService.update(input),
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_LIST_PROJECTS_QUERY_KEY], (old: ListProjectsOutput | undefined) => {
        if (!old) return { items: [] };
        return {
          items: old.items.map((oldProject): ListProjectItem => {
            if (oldProject.id !== data.id) return oldProject;

            const handleChangeValue = <T>(
              changeValue: { "UPDATE": T } | "REMOVE" | undefined,
              currentValue: T | undefined
            ): T | undefined => {
              if (changeValue === undefined) {
                return currentValue;
              }
              if (changeValue === "REMOVE") {
                return undefined;
              }
              if (typeof changeValue === "object" && "UPDATE" in changeValue) {
                return changeValue.UPDATE;
              }
              return currentValue;
            };

            const updatedIconPath = handleChangeValue(variables.iconPath, oldProject.iconPath);

            return {
              ...oldProject,
              ...variables,
              iconPath: updatedIconPath,
            };
          }),
        } satisfies ListProjectsOutput;
      });
    },
  });
};
