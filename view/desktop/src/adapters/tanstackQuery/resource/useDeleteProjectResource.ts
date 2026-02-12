import { resourceService } from "@/domains/resource/resourceService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ListProjectResourceItem } from "@repo/ipc";
import { DeleteResourceInput, DeleteResourceOutput } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "./useListProjectResources";

export interface UseDeleteProjectResourceInput {
  projectId: string;
  input: DeleteResourceInput;
}

export const useDeleteProjectResource = () => {
  const queryClient = useQueryClient();
  const { api } = useTabbedPaneStore();

  return useMutation<DeleteResourceOutput, Error, UseDeleteProjectResourceInput>({
    mutationFn: ({ projectId, input }) => resourceService.delete(projectId, input),
    onSuccess: async (data, variables) => {
      queryClient.setQueryData(
        [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: ListProjectResourceItem[]) => {
          const deletedResource = old.find((resource) => resource.id === data.id);

          if (!deletedResource) {
            return old;
          }

          return old.filter((resource) => {
            const panel = api?.getPanel(resource.id);

            if (resource.id === data.id) {
              if (panel) {
                api?.removePanel(panel);
              }
              return false;
            }

            if (resource.path.segments.length > deletedResource.path.segments.length) {
              const isNested = deletedResource.path.segments.every(
                (segment, index) => resource.path.segments[index] === segment
              );

              if (isNested) {
                if (panel) {
                  api?.removePanel(panel);
                }

                return false;
              }
            }

            return true;
          });
        }
      );
    },
  });
};
