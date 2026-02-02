import { TREE_VIEW_GROUP_ENVIRONMENTS, TREE_VIEW_GROUP_PROJECTS } from "@repo/moss-workspace";
import { ActivityBarItemState } from "./types";

export const defaultStates = [
  {
    id: TREE_VIEW_GROUP_PROJECTS,
    order: 1,
  },
  {
    id: TREE_VIEW_GROUP_ENVIRONMENTS,
    order: 2,
  },
  {
    id: "workbench.view.commit",
    order: 3,
  },
] as const satisfies ActivityBarItemState[];
