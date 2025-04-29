import { useQuery } from "@tanstack/react-query";

import { GroupView, MockGroupViews } from "../mockData";

export const USE_VIEW_GROUP_QUERY_KEY = "viewGroup";

export const getViewGroupFn = async (groupId: string): Promise<GroupView | null> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  return MockGroupViews[groupId as keyof typeof MockGroupViews] || null;
};

export const useGetViewGroup = (groupId: string) => {
  return useQuery<GroupView | null, Error>({
    queryKey: [USE_VIEW_GROUP_QUERY_KEY, groupId],
    queryFn: () => getViewGroupFn(groupId),
    enabled: !!groupId,
  });
};
