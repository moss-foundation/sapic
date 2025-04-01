import { type ActivityBarState, type ActivityBarPosition, MockActivityBarState } from "./mockData";

export type { ActivityBarState, ActivityBarPosition };

export const USE_ACTIVITY_BAR_STATE_QUERY_KEY = "activityBarState";
export const USE_CHANGE_ACTIVITY_BAR_STATE_MUTATION_KEY = "changeActivityBarState";

export let activityBarStateData: ActivityBarState = { ...MockActivityBarState };

export const getActivityBarStateFn = async (): Promise<ActivityBarState> => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return { ...activityBarStateData };
};

export const changeActivityBarStateFn = async (newState: Partial<ActivityBarState>): Promise<ActivityBarState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  activityBarStateData = {
    ...activityBarStateData,
    ...newState,
  };

  return { ...activityBarStateData };
};
