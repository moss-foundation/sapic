import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { StreamResourcesEvent, UpdateResourceInput, UpdateResourceOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_PROJECT_RESOURCE_QUERY_KEY } from "./useDescribeProjectResource";
import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "./useStreamProjectResources";

export interface UseUpdateProjectResourceInput {
  projectId: string;
  updatedResource: UpdateResourceInput;
}

const updateProjectResource = async ({ projectId, updatedResource }: UseUpdateProjectResourceInput) => {
  const result = await invokeTauriIpc<UpdateResourceOutput>("update_project_resource", {
    projectId: projectId,
    input: updatedResource,
  });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useUpdateProjectResource = () => {
  const queryClient = useQueryClient();

  return useMutation<UpdateResourceOutput, Error, UseUpdateProjectResourceInput>({
    mutationFn: updateProjectResource,
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
