import { projectService } from "@/domains/project/projectService";
import { ListProjectsOutput } from "@repo/ipc";
import { useQuery, useQueryClient } from "@tanstack/react-query";

export const USE_LIST_PROJECTS_QUERY_KEY = "listProjects";

export const useListProjects = () => {
  const queryClient = useQueryClient();

  const query = useQuery<ListProjectsOutput, Error>({
    queryKey: [USE_LIST_PROJECTS_QUERY_KEY],
    queryFn: projectService.listProjects,
  });

  const clearProjectsCacheAndRefetch = () => {
    queryClient.resetQueries({ queryKey: [USE_LIST_PROJECTS_QUERY_KEY] });
  };

  return {
    ...query,
    clearProjectsCacheAndRefetch,
  };
};
