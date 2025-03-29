import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  AppLayoutState,
  USE_APP_LAYOUT_STATE_QUERY_KEY,
  USE_CHANGE_APP_LAYOUT_STATE_MUTATION_KEY,
  changeAppLayoutStateFn,
} from "./useAppLayoutState";

export const useChangeAppLayoutState = () => {
  const queryClient = useQueryClient();

  return useMutation<AppLayoutState, Error, AppLayoutState>({
    mutationKey: [USE_CHANGE_APP_LAYOUT_STATE_MUTATION_KEY],
    mutationFn: changeAppLayoutStateFn,
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: [USE_APP_LAYOUT_STATE_QUERY_KEY] });
    },
  });
};
