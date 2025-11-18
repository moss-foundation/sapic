import { projectIpc } from "@/infra/ipc/project";
import { CreateProjectInput, CreateProjectOutput, StreamProjectsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

export const useCreateProject = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateProjectOutput, Error, CreateProjectInput>({
    mutationFn: (input) => projectIpc.createProject(input),
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAM_PROJECTS_QUERY_KEY], (old: StreamProjectsEvent[]) => {
        return [
          ...old,
          {
            ...inputToEvent(variables, data),
          },
        ];
      });
    },
  });
};

const inputToEvent = (input: CreateProjectInput, data: CreateProjectOutput): StreamProjectsEvent => {
  const { iconPath } = input;

  return {
    iconPath,
    archived: false,
    ...data,
  };
};
