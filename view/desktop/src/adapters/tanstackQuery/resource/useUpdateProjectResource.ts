import { ResourceDetails } from "@/db/resourceDetails/types";
import { resourceService } from "@/domains/resource/resourceService";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { UpdateResourceInput, UpdateResourceOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY } from "./useDescribeProjectResource";
import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "./useListProjectResources";

export interface UseUpdateProjectResourceInput {
  projectId: string;
  updateResourceInput: UpdateResourceInput;
}

export const useUpdateProjectResource = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateResourceOutput, Error, UseUpdateProjectResourceInput, ResourceDetails | undefined>({
    mutationFn: async ({ projectId, updateResourceInput }) => {
      return await resourceService.update(projectId, updateResourceInput);
    },

    onSuccess: async (data, variables) => {
      queryClient.setQueryData(
        [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: ListProjectResourcesOutput): ListProjectResourcesOutput => {
          const newResources = old.items.map((oldResource) => {
            const resourceDataFromBackend = "ITEM" in data ? data.ITEM : data.DIR;
            const updatedResourceData =
              "ITEM" in variables.updateResourceInput
                ? variables.updateResourceInput.ITEM
                : variables.updateResourceInput.DIR;
            if (oldResource.id === resourceDataFromBackend.id) {
              return {
                ...oldResource,
                ...updatedResourceData,
                ...resourceDataFromBackend,
              };
            }
            return oldResource;
          });

          return {
            ...old,
            items: newResources,
          };
        }
      );
      if ("ITEM" in data) {
        queryClient.invalidateQueries({
          queryKey: [USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY, variables.projectId, data.ITEM.id],
        });
      }
      if ("DIR" in data) {
        queryClient.invalidateQueries({
          queryKey: [USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY, variables.projectId, data.DIR.id],
        });
      }
    },
    onError(error) {
      console.error("Error updating project resource:", error);
    },
  });
};
