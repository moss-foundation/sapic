import { useMemo } from "react";

import { StreamEntriesEvent } from "@repo/moss-project";
import { StreamProjectsEvent } from "@repo/moss-workspace";
import { useQueries } from "@tanstack/react-query";

import { startStreamingProjectEntries } from "../queries/startStreamingProjectEntries";
import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY } from "../useStreamProjectEntries";
import { useStreamProjects } from "../useStreamProjects";

export interface ProjectWithEntries extends StreamProjectsEvent {
  entries: StreamEntriesEvent[];
  isEntriesLoading: boolean;
  entriesError?: Error | null;
}

export const useStreamedProjectsWithEntries = () => {
  const { data: projects = [], isLoading: areProjectsLoading, error: projectsError, ...query } = useStreamProjects();

  const entriesQueries = useQueries({
    queries: projects.map((project) => ({
      queryKey: [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, project.id],
      queryFn: () => startStreamingProjectEntries(project.id),
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

  const projectsWithEntries = useMemo((): ProjectWithEntries[] => {
    return projects.map((project, index) => {
      const entriesResult = entriesQueries.results[index];
      return {
        ...project,
        entries: entriesResult?.data || [],
        isEntriesLoading: entriesResult?.isPending || false,
        entriesError: entriesResult?.error || null,
      };
    });
  }, [projects, entriesQueries.results]);

  return {
    data: projectsWithEntries,
    isLoading: areProjectsLoading,
    error: projectsError,
    isEntriesLoading: entriesQueries.isLoading,
    hasEntriesError: entriesQueries.hasError,
    ...query,
  };
};
