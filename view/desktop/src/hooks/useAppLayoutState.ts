import { type AppLayoutState, MockAppLayoutState } from "./mockData";

export type { AppLayoutState };

export const USE_APP_LAYOUT_STATE_QUERY_KEY = "appLayoutState";
export const USE_CHANGE_APP_LAYOUT_STATE_MUTATION_KEY = "changeAppLayoutState";

export let appLayoutStateData: AppLayoutState = { ...MockAppLayoutState };

export const getAppLayoutStateFn = async (): Promise<AppLayoutState> => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return { ...appLayoutStateData };
};

export const changeAppLayoutStateFn = async (newLayout: Partial<AppLayoutState>): Promise<AppLayoutState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  appLayoutStateData = { ...appLayoutStateData, ...newLayout };
  return { ...appLayoutStateData };
};
