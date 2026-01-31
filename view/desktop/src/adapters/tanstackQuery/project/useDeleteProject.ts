import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { projectService } from "@/domains/project/projectService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { DeleteProjectInput, DeleteProjectOutput, StreamProjectsEvent } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useStreamedProjectsWithResources } from "./derivedHooks/useStreamedProjectsWithResources";
import { USE_STREAM_PROJECTS_QUERY_KEY } from "./useStreamProjects";

export interface UseDeleteProjectInput {
  id: string;
}

export const useDeleteProject = () => {
  const queryClient = useQueryClient();

  const { api } = useTabbedPaneStore();
  const { data: projectsWithResources } = useStreamedProjectsWithResources();

  return useMutation<DeleteProjectOutput, Error, DeleteProjectInput>({
    mutationFn: (input) => projectService.deleteProject(input),
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

      projectSummariesCollection.delete(data.id);
    },
  });
};
