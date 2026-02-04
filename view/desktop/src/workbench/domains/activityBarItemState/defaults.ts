import {
  ACTIVITY_BAR_VIEW_GROUP_ENVIRONMENTS_ID,
  ACTIVITY_BAR_VIEW_GROUP_PROJECTS_ID,
  ACTIVITY_BAR_VIEW_GROUP_SOURCE_CONTROL_ID,
} from "@/workbench/ui/parts/ActivityBar/constants";

import { ActivityBarItemState } from "./types";

export const defaultStates = [
  {
    id: ACTIVITY_BAR_VIEW_GROUP_PROJECTS_ID,
    order: 1,
  },
  {
    id: ACTIVITY_BAR_VIEW_GROUP_ENVIRONMENTS_ID,
    order: 2,
  },
  {
    id: ACTIVITY_BAR_VIEW_GROUP_SOURCE_CONTROL_ID,
    order: 3,
  },
] as const satisfies ActivityBarItemState[];
