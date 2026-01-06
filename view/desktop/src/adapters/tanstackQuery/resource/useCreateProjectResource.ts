import { resourceService } from "@/domains/resource/resourceService";
import { CreateResourceInput, CreateResourceOutput, StreamResourcesEvent } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { createProjectResourceForCache } from "../project/utils";
import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "./useStreamProjectResources";

export interface UseCreateProjectResourceInputProps {
  projectId: string;
  input: CreateResourceInput;
}

export const useCreateProjectResource = () => {
  const queryClient = useQueryClient();

  return useMutation<CreateResourceOutput, Error, UseCreateProjectResourceInputProps>({
    mutationFn: ({ projectId, input }) => resourceService.create(projectId, input),
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
