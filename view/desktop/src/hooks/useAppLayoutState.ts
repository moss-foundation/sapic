// FIXME: remove mock data
export interface AppLayoutState {
  activeSidebar: "left" | "right" | "none";
  sidebarSetting: "left" | "right";
}

export const USE_APP_LAYOUT_STATE_QUERY_KEY = "appLayoutState";
export const USE_CHANGE_APP_LAYOUT_STATE_MUTATION_KEY = "changeAppLayoutState";

export let AppLayoutState: AppLayoutState = {
  activeSidebar: "left",
  sidebarSetting: "left",
};

export const getAppLayoutStateFn = async (): Promise<AppLayoutState> => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return { ...AppLayoutState };
};

export const changeAppLayoutStateFn = async (newLayout: Partial<AppLayoutState>): Promise<AppLayoutState> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  AppLayoutState = { ...AppLayoutState, ...newLayout };
  return { ...AppLayoutState };
};
