import { useQuery } from "@tanstack/react-query";
import { AppLayoutState, USE_APP_LAYOUT_STATE_QUERY_KEY, getAppLayoutStateFn } from "./useAppLayoutState";

export const useGetAppLayoutState = () => {
  return useQuery<AppLayoutState, Error>({
    queryKey: [USE_APP_LAYOUT_STATE_QUERY_KEY],
    queryFn: getAppLayoutStateFn,
  });
};
