import { statusBarItemStateService } from "@/workbench/services/statusBarItemState/service";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY = "getStatusBarItemState" as const;

export const useGetStatusBarItemState = (id: string) => {
  return useQuery<number | undefined, Error>({
    queryKey: [USE_GET_STATUS_BAR_ITEM_STATE_QUERY_KEY, id],
    queryFn: () => statusBarItemStateService.get(id),
  });
};
