import { useMutation, useQueryClient } from "@tanstack/react-query";

import { MockViews, Views } from "../mockData";
import { USE_VIEW_GROUPS_QUERY_KEY } from "./useGetViewGroups";

export const USE_CHANGE_VIEW_GROUPS_MUTATION_KEY = "changeViewGroups";
export let viewsData: Views = MockViews;

export const changeViewGroupsFn = async (newViewGroups: Views): Promise<Views> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  viewsData = newViewGroups;

  return newViewGroups;
};

export const useChangeViewGroups = () => {
  const queryClient = useQueryClient();

  return useMutation<Views, Error, Views>({
    mutationKey: [USE_CHANGE_VIEW_GROUPS_MUTATION_KEY],
    mutationFn: changeViewGroupsFn,
    onSuccess(newViewGroups) {
      queryClient.setQueryData([USE_VIEW_GROUPS_QUERY_KEY], newViewGroups);
    },
  });
};
