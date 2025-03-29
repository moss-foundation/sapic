import { useQuery } from "@tanstack/react-query";
import { GroupView, USE_VIEW_GROUP_QUERY_KEY, getViewGroupFn } from "./useViewGroups";

export const useGetViewGroup = (groupId: string) => {
  return useQuery<GroupView | null, Error>({
    queryKey: [USE_VIEW_GROUP_QUERY_KEY, groupId],
    queryFn: () => getViewGroupFn(groupId),
    enabled: !!groupId,
  });
};
