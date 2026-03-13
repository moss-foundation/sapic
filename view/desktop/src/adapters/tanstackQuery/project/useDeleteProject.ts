import { projectService } from "@/domains/project/projectService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { DeleteProjectInput, DeleteProjectOutput, ListProjectsOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useProjectsWithResources } from "./derivedHooks/useProjectsWithResources";
import { USE_LIST_PROJECTS_QUERY_KEY } from "./useListProjects";

export interface UseDeleteProjectInput {
  id: string;
}

export const useDeleteProject = () => {
  const queryClient = useQueryClient();

  const { api } = useTabbedPaneStore();
  const { data: projectsWithResources } = useProjectsWithResources();

  return useMutation<DeleteProjectOutput, Error, DeleteProjectInput>({
    mutationFn: (input) => projectService.delete(input),
    onSuccess: (data) => {
      queryClient.setQueryData([USE_LIST_PROJECTS_QUERY_KEY], (old: ListProjectsOutput | undefined) => {
        return {
          items: old?.items.filter((project) => project.id !== data.id) ?? [],
        } satisfies ListProjectsOutput;
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
