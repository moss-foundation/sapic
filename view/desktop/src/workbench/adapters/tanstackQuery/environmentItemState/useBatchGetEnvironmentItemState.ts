import { environmentItemStateService } from "@/workbench/domains/environmentItemState/service";
import { EnvironmentItemState } from "@/workbench/domains/environmentItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_BATCH_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY = "batchGetEnvironmentItemState" as const;

export const useBatchGetEnvironmentItemState = (ids: string[], workspaceId: string) => {
  return useQuery<EnvironmentItemState[], Error>({
    queryKey: [USE_BATCH_GET_ENVIRONMENT_ITEM_STATE_QUERY_KEY, ids, workspaceId],
    queryFn: () => environmentItemStateService.batchGet(ids, workspaceId),
  });
};
