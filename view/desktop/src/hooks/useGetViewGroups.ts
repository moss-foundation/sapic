import { useQuery } from "@tanstack/react-query";
import { Views, USE_VIEW_GROUPS_QUERY_KEY, getViewGroupsFn } from "./useViewGroups";

export const useGetViewGroups = () => {
  return useQuery<Views, Error>({
    queryKey: [USE_VIEW_GROUPS_QUERY_KEY],
    queryFn: getViewGroupsFn,
  });
};
