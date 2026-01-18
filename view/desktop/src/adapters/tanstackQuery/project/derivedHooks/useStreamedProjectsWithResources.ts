import { useMemo } from "react";

import { StreamProjectsEvent } from "@repo/ipc";
import { StreamResourcesEvent } from "@repo/moss-project";
import { useQueries } from "@tanstack/react-query";

import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "../../resource/useStreamProjectResources";
import { startStreamingProjectResources } from "../queries/startStreamingProjectResources";
import { useStreamProjects } from "../useStreamProjects";

export interface ProjectWithResources extends StreamProjectsEvent {
  resources: StreamResourcesEvent[];
  areResourcesLoading: boolean;
  resourcesError?: Error | null;
}

export const useStreamedProjectsWithResources = () => {
  const { data: projects = [], isLoading: areProjectsLoading, error: projectsError, ...query } = useStreamProjects();

  const resourcesQueries = useQueries({
    queries: projects.map((project) => ({
      queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, project.id],
      queryFn: () => startStreamingProjectResources(project.id),
      placeholderData: [],
    })),
    combine: (results) => {
      return {
        data: results.map((result) => result.data || []),
        isLoading: results.some((result) => result.isPending),
        hasError: results.some((result) => result.error),
        results: results,
      };
    },
  });

  const projectsWithResources = useMemo((): ProjectWithResources[] => {
    return projects.map((project, index) => {
      const resourcesResult = resourcesQueries.results[index];
      return {
        ...project,
        resources: resourcesResult?.data || [],
        areResourcesLoading: resourcesResult?.isPending || false,
        resourcesError: resourcesResult?.error || null,
      };
    });
  }, [projects, resourcesQueries.results]);

  return {
    data: projectsWithResources,
    isLoading: areProjectsLoading,
    error: projectsError,
    areResourcesLoading: resourcesQueries.isLoading,
    hasResourcesError: resourcesQueries.hasError,
    ...query,
  };
};
