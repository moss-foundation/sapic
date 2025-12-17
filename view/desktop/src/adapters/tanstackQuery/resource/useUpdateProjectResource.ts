import { resourceService } from "@/domains/resource/resourceService";
import { StreamResourcesEvent, UpdateResourceInput, UpdateResourceOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY } from "./useDescribeProjectResource";
import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "./useStreamProjectResources";

export interface UseUpdateProjectResourceInput {
  projectId: string;
  updatedResource: UpdateResourceInput;
}

export const useUpdateProjectResource = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateResourceOutput, Error, UseUpdateProjectResourceInput>({
    mutationFn: ({ projectId, updatedResource }) => resourceService.update(projectId, updatedResource),
    onSuccess: async (data, variables) => {
      queryClient.setQueryData(
        [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: StreamResourcesEvent[]) => {
          return old.map((oldResource) => {
            const resourceDataFromBackend = "ITEM" in data ? data.ITEM : data.DIR;
            const updatedResourceData =
              "ITEM" in variables.updatedResource ? variables.updatedResource.ITEM : variables.updatedResource.DIR;

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
        // queryClient.setQueryData(
        //   [USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY, variables.projectId, data.DIR.id],
        //   (old: DescribeResourceOutput) => {
        //     if (!old) return old;

        //     //TODO: this is a temporary solution to preserve the existing URL from cache
        //     // Preserve the existing URL if it exists and is not "Hardcoded Value"
        //     const preservedUrl = old.url && old.url !== "Hardcoded Value" ? old.url : undefined;

        //     const { url: _url, ...rest } = old;
        //     return {
        //       ...rest,
        //       ...data.DIR,
        //       // Only set URL if we have a preserved one, otherwise keep it undefined
        //       ...(preservedUrl !== undefined && { url: preservedUrl }),
        //     };
        //   }
        // );
      }
    },
    onError(error) {
      console.error("Error updating project resource:", error);
    },
  });
};
