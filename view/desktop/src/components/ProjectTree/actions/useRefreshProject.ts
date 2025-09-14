import { USE_STREAM_PROJECT_ENTRIES_QUERY_KEY } from "@/hooks";
import { useFetchEntriesForPath } from "@/hooks/project/derivedHooks/useFetchEntriesForPath";
import { useQueryClient } from "@tanstack/react-query";

export const useRefreshProject = (projectId: string) => {
  const queryClient = useQueryClient();

  const { fetchEntriesForPath } = useFetchEntriesForPath();

  const refreshProject = async () => {
    queryClient.invalidateQueries({
      queryKey: [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, projectId],
    });
    queryClient.removeQueries({ queryKey: [USE_STREAM_PROJECT_ENTRIES_QUERY_KEY, projectId] });

    await fetchEntriesForPath(projectId, "");
  };

  return { refreshProject };
};
