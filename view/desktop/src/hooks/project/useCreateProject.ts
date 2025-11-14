import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { CreateProjectInput, CreateProjectOutput, StreamProjectsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

const createProject = async (input: CreateProjectInput) => {
  const result = await invokeTauriIpc<CreateProjectOutput>("create_project", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateProject = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createProject,
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
