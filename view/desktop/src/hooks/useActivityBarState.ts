// FIXME: remove mock data
export type ActivityBarPosition = "top" | "bottom" | "hidden" | "default";

export interface ActivityBarState {
  position: ActivityBarPosition;
  groupOrder: string[];
}

export const USE_ACTIVITY_BAR_STATE_QUERY_KEY = "activityBarState";
export const USE_CHANGE_ACTIVITY_BAR_STATE_MUTATION_KEY = "changeActivityBarState";

export let ActivityBarState: ActivityBarState = {
  position: "default",
  groupOrder: [],
};

export const getActivityBarStateFn = async (): Promise<ActivityBarState> => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return { ...ActivityBarState };
};

export const changeActivityBarStateFn = async (newState: Partial<ActivityBarState>): Promise<ActivityBarState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  ActivityBarState = {
    ...ActivityBarState,
    ...newState,
  };

  return { ...ActivityBarState };
};
