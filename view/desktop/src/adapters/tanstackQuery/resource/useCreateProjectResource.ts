import { resourceService } from "@/domains/resource/resourceService";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { CreateResourceInput, CreateResourceOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { createProjectResourceForCache } from "../project/utils";
import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "./useListProjectResources";

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
        [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: ListProjectResourcesOutput): ListProjectResourcesOutput => {
          const newResources = [...old.items, newResource];

          return {
            ...old,
            items: newResources,
          };
        }
      );
    },
  });
};
