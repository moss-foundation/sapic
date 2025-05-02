import { useQuery } from "@tanstack/react-query";

import { MockViews, Views } from "../mockData";

export const USE_VIEW_GROUPS_QUERY_KEY = "viewGroups";

export const viewsData: Views = MockViews;

export const getViewGroupsFn = async (): Promise<Views> => {
  await new Promise((resolve) => setTimeout(resolve, 50));
  return viewsData;
};

export const useGetViewGroups = () => {
  return useQuery<Views, Error>({
    queryKey: [USE_VIEW_GROUPS_QUERY_KEY],
    queryFn: getViewGroupsFn,
  });
};
