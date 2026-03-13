import { useMemo } from "react";

import { resourceService } from "@/domains/resource/resourceService";
import { ListProjectItem, ListProjectResourceItem } from "@repo/ipc";
import { useQueries } from "@tanstack/react-query";

import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "../../resource/useListProjectResources";
import { useListProjects } from "../useListProjects";

export interface ProjectWithResources extends ListProjectItem {
  resources: ListProjectResourceItem[];
  areResourcesLoading: boolean;
  resourcesError?: Error | null;
}

export const useProjectsWithResources = () => {
  const {
    data: projects = { items: [] },
    isLoading: areProjectsLoading,
    error: projectsError,
    ...query
  } = useListProjects();

  const resourcesQueries = useQueries({
    queries: projects?.items.map((project) => ({
      queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY, project.id],
      queryFn: () => resourceService.list({ projectId: project.id, mode: "LOAD_ROOT" }),
      placeholderData: { items: [] },
    })),
    combine: (results) => {
      return {
        data: results.map((result) => result.data?.items || []),
        isLoading: results.some((result) => result.isPending),
        hasError: results.some((result) => result.error),
        results: results,
      };
    },
  });

  const projectsWithResources = useMemo((): ProjectWithResources[] => {
    return projects?.items.map((project, index) => {
      const resourcesResult = resourcesQueries.results[index];

      return {
        ...project,
        resources: resourcesResult?.data?.items || [],
        areResourcesLoading: resourcesResult?.isPending || false,
        resourcesError: resourcesResult?.error || null,
      };
    });
  }, [projects?.items, resourcesQueries.results]);

  return {
    data: projectsWithResources,
    isLoading: areProjectsLoading,
    error: projectsError,
    areResourcesLoading: resourcesQueries.isLoading,
    hasResourcesError: resourcesQueries.hasError,
    ...query,
  };
};
