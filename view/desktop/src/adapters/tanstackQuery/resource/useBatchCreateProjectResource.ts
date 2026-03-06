import { resourceService } from "@/domains/resource/resourceService";
import { ListProjectResourcesOutput } from "@repo/ipc";
import { BatchCreateResourceInput, BatchCreateResourceOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { createProjectResourceForCache } from "../project/utils";
import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "./useListProjectResources";

export interface UseBatchCreateProjectResourceInput {
  projectId: string;
  input: BatchCreateResourceInput;
}

export const useBatchCreateProjectResource = () => {
  const queryClient = useQueryClient();

  return useMutation<BatchCreateResourceOutput, Error, UseBatchCreateProjectResourceInput>({
    mutationFn: ({ projectId, input }) => resourceService.batchCreate(projectId, input),
    onSuccess: async (data, variables) => {
      const newResources = await Promise.all(
        data.resources.map((created) => {
          const matchingInput = variables.input.resources.find((input) => {
            if ("ITEM" in input) {
              return input.ITEM.path === created.path.raw && input.ITEM.name === created.name;
            } else {
              return input.DIR.path === created.path.raw && input.DIR.name === created.name;
            }
          });

          return createProjectResourceForCache(created.id, matchingInput!);
        })
      );

      queryClient.setQueryData(
        [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old?: ListProjectResourcesOutput): ListProjectResourcesOutput => {
          return {
            ...old,
            items: [...(old?.items ?? []), ...newResources],
          };
        }
      );
    },
  });
};
