// FIXME: remove mock data
export interface AppLayoutState {
  activeSidebar: "left" | "right" | "none";
}

export const USE_APP_LAYOUT_STATE_QUERY_KEY = "appLayoutState";
export const USE_CHANGE_APP_LAYOUT_STATE_MUTATION_KEY = "changeAppLayoutState";

export let AppLayoutState = {
  activeSidebar: "left",
};

export const getAppLayoutStateFn = async (): Promise<AppLayoutState> => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return AppLayoutState as AppLayoutState;
};

export const changeAppLayoutStateFn = async (newLayout: AppLayoutState): Promise<AppLayoutState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  AppLayoutState = newLayout;
  return newLayout;
};
