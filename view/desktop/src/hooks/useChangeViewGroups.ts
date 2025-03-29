import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  Views,
  USE_VIEW_GROUPS_QUERY_KEY,
  USE_CHANGE_VIEW_GROUPS_MUTATION_KEY,
  changeViewGroupsFn,
} from "./useViewGroups";

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
