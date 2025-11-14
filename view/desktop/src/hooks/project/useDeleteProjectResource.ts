import { invokeTauriIpc } from "@/lib/backend/tauri";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { DeleteResourceInput, DeleteResourceOutput, StreamResourcesEvent } from "@repo/moss-project";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "./useStreamProjectResources";

export interface UseDeleteProjectResourceInput {
  projectId: string;
  input: DeleteResourceInput;
}

const deleteProjectResource = async ({ projectId, input }: UseDeleteProjectResourceInput) => {
  const result = await invokeTauriIpc<DeleteResourceOutput>("delete_project_resource", { projectId: projectId, input });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteProjectResource = () => {
  const queryClient = useQueryClient();
  const { api } = useTabbedPaneStore();

  return useMutation<DeleteResourceOutput, Error, UseDeleteProjectResourceInput>({
    mutationFn: deleteProjectResource,
    onSuccess: async (data, variables) => {
      queryClient.setQueryData(
        [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, variables.projectId],
        (old: StreamResourcesEvent[]) => {
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
