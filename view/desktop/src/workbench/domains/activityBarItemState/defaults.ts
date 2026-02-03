import {
  PLACEHOLDER_ACTIVITY_BAR_VIEW_GROUP_ENVIRONMENTS,
  PLACEHOLDER_ACTIVITY_BAR_VIEW_GROUP_PROJECTS,
  PLACEHOLDER_ACTIVITY_BAR_VIEW_GROUP_SOURCE_CONTROL,
} from "@/workbench/ui/parts/ActivityBar/constants";

import { ActivityBarItemState } from "./types";

export const defaultStates = [
  {
    id: PLACEHOLDER_ACTIVITY_BAR_VIEW_GROUP_PROJECTS,
    order: 1,
  },
  {
    id: PLACEHOLDER_ACTIVITY_BAR_VIEW_GROUP_ENVIRONMENTS,
    order: 2,
  },
  {
    id: PLACEHOLDER_ACTIVITY_BAR_VIEW_GROUP_SOURCE_CONTROL,
    order: 3,
  },
] as const satisfies ActivityBarItemState[];
