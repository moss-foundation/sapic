import { invokeTauriIpc } from "@/infra/ipc/tauri";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { DeleteProjectInput, DeleteProjectOutput, StreamProjectsEvent } from "@repo/moss-workspace";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useStreamedProjectsWithResources } from "..";
import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

export interface UseDeleteProjectInput {
  id: string;
}

const deleteStreamedProject = async ({ id }: DeleteProjectInput) => {
  const result = await invokeTauriIpc<DeleteProjectOutput>("delete_project", { input: { id } });

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useDeleteProject = () => {
  const queryClient = useQueryClient();
  const { api } = useTabbedPaneStore();
  const { data: projectsWithResources } = useStreamedProjectsWithResources();

  return useMutation({
    mutationFn: deleteStreamedProject,
    onSuccess: (data) => {
      queryClient.setQueryData([USE_STREAM_PROJECTS_QUERY_KEY], (old: StreamProjectsEvent[]) => {
        return old.filter((project) => project.id !== data.id);
      });

      projectsWithResources?.forEach((project) => {
        if (project.id === data.id) {
          const projectPanel = api?.getPanel(project.id);

          if (projectPanel) {
            api?.removePanel(projectPanel);
          }

          project.resources.forEach((resource) => {
            const panel = api?.getPanel(resource.id);
            if (panel) {
              api?.removePanel(panel);
            }
          });
        }
      });
    },
  });
};
