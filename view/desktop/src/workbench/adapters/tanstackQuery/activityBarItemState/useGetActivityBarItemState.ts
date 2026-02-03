import { activityBarItemStateService } from "@/workbench/domains/activityBarItemState/service";
import { ActivityBarItemState } from "@/workbench/domains/activityBarItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY = "getActivityBarItemState";

export const useGetActivityBarItemState = (id: string) => {
  return useQuery<ActivityBarItemState, Error>({
    queryKey: [USE_GET_ACTIVITY_BAR_ITEM_STATE_QUERY_KEY, id],
    queryFn: () => activityBarItemStateService.get(id),
  });
};
