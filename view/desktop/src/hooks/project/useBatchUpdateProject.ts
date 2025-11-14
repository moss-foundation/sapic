import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { BatchUpdateProjectInput, StreamProjectsEvent, UpdateProjectOutput } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

const batchUpdateProject = async (input: BatchUpdateProjectInput) => {
  const result = await invokeTauriIpc<UpdateProjectOutput>("batch_update_project", {
    input,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useBatchUpdateProject = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: batchUpdateProject,
    onSuccess: (_, variables) => {
      queryClient.setQueryData([USE_STREAM_PROJECTS_QUERY_KEY], (old: StreamProjectsEvent[]) => {
        return old.map((oldProject) => {
          const updatedProject = variables.items.find((project) => project.id === oldProject.id);
          if (updatedProject) {
            return {
              ...oldProject,
              ...updatedProject,
            };
          }

          return oldProject;
        });
      });
    },
  });
};
