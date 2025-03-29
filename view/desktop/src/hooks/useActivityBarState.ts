import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export type ActivityBarPosition = "top" | "bottom" | "hidden" | "default";

export interface ActivityBarState {
  position: ActivityBarPosition;
  groupOrder: string[];
}

export const USE_ACTIVITY_BAR_STATE_QUERY_KEY = "activityBarState";
export const USE_CHANGE_ACTIVITY_BAR_STATE_MUTATION_KEY = "changeActivityBarState";

let ActivityBarState: ActivityBarState = {
  position: "default",
  groupOrder: [],
};

const getActivityBarStateFn = async (): Promise<ActivityBarState> => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return { ...ActivityBarState };
};

const changeActivityBarStateFn = async (newState: Partial<ActivityBarState>): Promise<ActivityBarState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  ActivityBarState = {
    ...ActivityBarState,
    ...newState,
  };

  return { ...ActivityBarState };
};

export const useGetActivityBarState = () => {
  return useQuery<ActivityBarState, Error>({
    queryKey: [USE_ACTIVITY_BAR_STATE_QUERY_KEY],
    queryFn: getActivityBarStateFn,
  });
};

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
