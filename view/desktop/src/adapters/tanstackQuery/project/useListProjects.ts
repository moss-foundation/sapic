import { projectService } from "@/domains/project/projectService";
import { ListProjectsOutput } from "@repo/ipc";
import { useQuery } from "@tanstack/react-query";

export const USE_LIST_PROJECTS_QUERY_KEY = "listProjects";

export const useListProjects = () => {
  return useQuery<ListProjectsOutput, Error>({
    queryKey: [USE_LIST_PROJECTS_QUERY_KEY],
    queryFn: projectService.list,
  });
};
