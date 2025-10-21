import { USE_STREAM_PROJECT_RESOURCES_QUERY_KEY } from "@/hooks";
import { useFetchResourcesForPath } from "@/hooks/project/derivedHooks/useFetchResourceForPath";
import { useQueryClient } from "@tanstack/react-query";

export const useRefreshProject = (projectId: string) => {
  const queryClient = useQueryClient();

  const { fetchResourcesForPath } = useFetchResourcesForPath();

  const refreshProject = async () => {
    queryClient.invalidateQueries({
      queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, projectId],
    });
    queryClient.removeQueries({ queryKey: [USE_STREAM_PROJECT_RESOURCES_QUERY_KEY, projectId] });

    await fetchResourcesForPath(projectId, "");
  };

  return { refreshProject };
};
