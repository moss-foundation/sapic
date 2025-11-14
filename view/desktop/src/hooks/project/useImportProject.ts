import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { ImportProjectInput, ImportProjectOutput, StreamProjectsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

export const IMPORT_PROJECT_QUERY_KEY = "importProject";

const importProject = async (input: ImportProjectInput) => {
  const result = await invokeTauriIpc<ImportProjectOutput>("import_project", { input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useImportProject = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationKey: [IMPORT_PROJECT_QUERY_KEY],
    mutationFn: importProject,
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
