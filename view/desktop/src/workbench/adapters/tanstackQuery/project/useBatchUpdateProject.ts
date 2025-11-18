import { projectIpc } from "@/infra/ipc/project";
import { BatchUpdateProjectInput, BatchUpdateProjectOutput, StreamProjectsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

export const useBatchUpdateProject = () => {
  const queryClient = useQueryClient();

  return useMutation<BatchUpdateProjectOutput, Error, BatchUpdateProjectInput>({
    mutationFn: (input) => projectIpc.batchUpdateProject(input),
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
