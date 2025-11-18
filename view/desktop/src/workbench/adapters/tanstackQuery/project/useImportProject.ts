import { projectIpc } from "@/infra/ipc/project";
import { ImportProjectInput, ImportProjectOutput, StreamProjectsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

export const IMPORT_PROJECT_QUERY_KEY = "importProject";

export const useImportProject = () => {
  const queryClient = useQueryClient();

  return useMutation<ImportProjectOutput, Error, ImportProjectInput>({
    mutationKey: [IMPORT_PROJECT_QUERY_KEY],
    mutationFn: (input) => projectIpc.importProject(input),
    onSuccess: (data, variables) => {
      queryClient.setQueryData([USE_STREAM_PROJECTS_QUERY_KEY], (old: StreamProjectsEvent[]) => {
        return [
          ...old,
          {
            ...data,
            ...variables,
          },
        ];
      });
    },
  });
};
