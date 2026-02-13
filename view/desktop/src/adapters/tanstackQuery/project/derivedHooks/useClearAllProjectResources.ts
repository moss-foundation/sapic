import { useQueryClient } from "@tanstack/react-query";

import { USE_LIST_PROJECT_RESOURCES_QUERY_KEY } from "../../resource/useListProjectResources";

export const useClearAllProjectResources = () => {
  const queryClient = useQueryClient();

  const clearAllProjectResourcesCache = () => {
    queryClient.removeQueries({
      queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY],
      exact: false,
    });

    queryClient.invalidateQueries({
      queryKey: [USE_LIST_PROJECT_RESOURCES_QUERY_KEY],
      exact: false,
    });
  };

  return {
    clearAllProjectResourcesCache,
  };
};
