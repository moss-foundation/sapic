import { projectService } from "@/domains/project/projectService";
import { StreamProjectsEvent, UpdateProjectInput, UpdateProjectOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

export const useUpdateProject = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateProjectOutput, Error, UpdateProjectInput>({
    mutationFn: (input) => projectService.updateProject(input),
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAM_PROJECTS_QUERY_KEY], (old: StreamProjectsEvent[]) => {
        return old.map((oldProject) => {
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
        });
      });
    },
  });
};
