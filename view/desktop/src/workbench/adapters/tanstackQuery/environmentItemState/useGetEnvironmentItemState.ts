import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY = "getEnvironmentItemState" as const;

export const useGetEnvironmentItemState = (id: string, workspaceId: string) => {
  return useQuery<EnvironmentItemState, Error>({
    queryKey: [USE_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY, id, workspaceId],
    queryFn: () => environmentItemStateService.get(id, workspaceId),
  });
};
