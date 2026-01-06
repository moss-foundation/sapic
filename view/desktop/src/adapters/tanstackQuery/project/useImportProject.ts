import { projectService } from "@/domains/project/projectService";
import { ImportProjectInput, ImportProjectOutput, StreamProjectsEvent } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

export const IMPORT_PROJECT_QUERY_KEY = "importProject";

export const useImportProject = () => {
  const queryClient = useQueryClient();

  return useMutation<ImportProjectOutput, Error, ImportProjectInput>({
    mutationKey: [IMPORT_PROJECT_QUERY_KEY],
    mutationFn: (input) => projectService.importProject(input),
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
