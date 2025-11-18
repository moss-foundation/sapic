import { projectIpc } from "@/infra/ipc/project";
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
    mutationFn: ({ projectId, updatedResource }) => projectIpc.updateProjectResource(projectId, updatedResource),
    onSuccess: async (data, variables) => {
      queryClient.setQueryData(
        [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: StreamResourcesEvent[]) => {
          return old.map((oldResource) => {
            const resourceDataFromBackend = "ITEM" in data ? data.ITEM : data.DIR;
            const payloadResourceData =
              "ITEM" in variables.updatedResource ? variables.updatedResource.ITEM : variables.updatedResource.DIR;

            if (oldResource.id === resourceDataFromBackend.id) {
              return {
                ...oldResource,
                ...payloadResourceData,
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
  });
};
