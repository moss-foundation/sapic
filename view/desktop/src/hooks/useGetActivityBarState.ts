import { useQuery } from "@tanstack/react-query";
import { ActivityBarState, USE_ACTIVITY_BAR_STATE_QUERY_KEY, getActivityBarStateFn } from "./useActivityBarState";

export const useGetActivityBarState = () => {
  return useQuery<ActivityBarState, Error>({
    queryKey: [USE_ACTIVITY_BAR_STATE_QUERY_KEY],
    queryFn: getActivityBarStateFn,
  });
};
