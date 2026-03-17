import { projectService } from "@/domains/project/projectService";
import { CreateProjectInput, CreateProjectOutput, ListProjectItem, ListProjectsOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_PROJECTS_QUERY_KEY } from "./useListProjects";

export const useCreateProject = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateProjectOutput, Error, CreateProjectInput>({
    mutationFn: (input) => projectService.create(input),
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_LIST_PROJECTS_QUERY_KEY], (old: ListProjectsOutput | undefined) => {
        return {
          items: [
            ...(old?.items ?? []),
            {
              ...inputToEvent(variables, data),
            },
          ],
        } satisfies ListProjectsOutput;
      });
    },
  });
};

const inputToEvent = (input: CreateProjectInput, data: CreateProjectOutput): ListProjectItem => {
  const { iconPath } = input;

  return {
    iconPath,
    archived: false,
    ...data,
  };
};
