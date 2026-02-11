import { projectService } from "@/domains/project/projectService";
import { ImportProjectInput, ImportProjectOutput, ListProjectItem, ListProjectsOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_PROJECTS_QUERY_KEY } from "./useListProjects";

export const IMPORT_PROJECT_QUERY_KEY = "importProject";

export const useImportProject = () => {
  const queryClient = useQueryClient();

  return useMutation<ImportProjectOutput, Error, ImportProjectInput>({
    mutationKey: [IMPORT_PROJECT_QUERY_KEY],
    mutationFn: (input) => projectService.importProject(input),
    onSuccess: (data) => {
      queryClient.setQueryData([USE_LIST_PROJECTS_QUERY_KEY], (old: ListProjectsOutput | undefined) => {
        const newItem: ListProjectItem = {
          id: data.id,
          name: data.name,
          iconPath: data.iconPath,
          archived: false,
        };
        return {
          items: [...(old?.items ?? []), newItem],
        };
      });
    },
  });
};
