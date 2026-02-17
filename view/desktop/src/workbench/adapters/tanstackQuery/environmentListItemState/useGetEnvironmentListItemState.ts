import { environmentListItemStateService } from "@/workbench/domains/environmentListItemState/service";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY = "getEnvironmentListItemState" as const;

export const useGetEnvironmentListItemState = (id: string, workspaceId: string) => {
  return useQuery<boolean | undefined, Error>({
    queryKey: [USE_GET_ENVIRONMENT_LIST_ITEM_STATE_QUERY_KEY, id, workspaceId],
    queryFn: () => environmentListItemStateService.getExpanded(id, workspaceId),
  });
};
