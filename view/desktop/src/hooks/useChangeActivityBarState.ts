import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  ActivityBarState,
  USE_ACTIVITY_BAR_STATE_QUERY_KEY,
  USE_CHANGE_ACTIVITY_BAR_STATE_MUTATION_KEY,
  changeActivityBarStateFn,
} from "./useActivityBarState";

export const useChangeActivityBarState = () => {
  const queryClient = useQueryClient();

  return useMutation<ActivityBarState, Error, Partial<ActivityBarState>>({
    mutationKey: [USE_CHANGE_ACTIVITY_BAR_STATE_MUTATION_KEY],
    mutationFn: changeActivityBarStateFn,
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: [USE_ACTIVITY_BAR_STATE_QUERY_KEY] });
    },
  });
};
