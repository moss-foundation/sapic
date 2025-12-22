import { resourceDetailsCollection } from "@/db/resource/resourceDetailsCollection";
import { ResourceDetails } from "@/db/resource/types";
import { resourceService } from "@/domains/resource/resourceService";
import { StreamResourcesEvent, UpdateResourceInput, UpdateResourceOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { mapUpdateResourceInputToResourceDetails } from "./mappers";
import { USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY } from "./useDescribeProjectResource";
import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "./useStreamProjectResources";

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
    onMutate: async ({ updateResourceInput }): Promise<ResourceDetails | undefined> => {
      const resourceId = "ITEM" in updateResourceInput ? updateResourceInput.ITEM.id : updateResourceInput.DIR?.id;
      const existingResourceDetails = resourceDetailsCollection.get(resourceId);

      const updatedResourceDetails = mapUpdateResourceInputToResourceDetails(
        updateResourceInput,
        existingResourceDetails
      );

      resourceDetailsCollection.update(updatedResourceDetails.id, (draft) => {
        if (!draft) return;
        Object.assign(draft, {
          ...updatedResourceDetails,
          metadata: { isDirty: false },
        });
      });

      return existingResourceDetails;
    },
    onSuccess: async (data, variables) => {
      queryClient.setQueryData(
        [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: StreamResourcesEvent[]) => {
          return old.map((oldResource) => {
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
    onError(error, _variables, context) {
      console.error("Error updating project resource:", error);
      if (!context) return;

      resourceDetailsCollection.update(context.id, (draft) => {
        if (!draft) return;
        Object.assign(draft, context);
      });
    },
  });
};
