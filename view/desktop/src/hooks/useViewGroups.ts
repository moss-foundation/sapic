import { type ViewGroup, type Views, type GroupView, MockViews, MockGroupViews } from "./mockData";

export type { ViewGroup, Views, GroupView };

export const USE_VIEW_GROUPS_QUERY_KEY = "viewGroups";
export const USE_CHANGE_VIEW_GROUPS_MUTATION_KEY = "changeViewGroups";
export const USE_VIEW_GROUP_QUERY_KEY = "viewGroup";

export let viewsData: Views = MockViews;

export const getViewGroupsFn = async (): Promise<Views> => {
  await new Promise((resolve) => setTimeout(resolve, 50));
  return viewsData;
};

export const changeViewGroupsFn = async (newViewGroups: Views): Promise<Views> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  viewsData = newViewGroups;

  return newViewGroups;
};

export const getViewGroupFn = async (groupId: string): Promise<GroupView | null> => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  return MockGroupViews[groupId as keyof typeof MockGroupViews] || null;
};
