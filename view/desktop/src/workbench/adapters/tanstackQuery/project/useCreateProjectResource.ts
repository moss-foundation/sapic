import { projectIpc } from "@/infra/ipc/project";
import { CreateResourceInput, CreateResourceOutput, StreamResourcesEvent } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "./useStreamProjectResources";
import { createProjectResourceForCache } from "./utils";

export interface UseCreateProjectResourceInputProps {
  projectId: string;
  input: CreateResourceInput;
}

export const useCreateProjectResource = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateResourceOutput, Error, UseCreateProjectResourceInputProps>({
    mutationFn: ({ projectId, input }) => projectIpc.createProjectResource(projectId, input),
    onSuccess: async (data, variables) => {
      const newResource = await createProjectResourceForCache(data.id, variables.input);

      queryClient.setQueryData(
        [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: StreamResourcesEvent[]) => {
          return [...old, newResource];
        }
      );
    },
  });
};
