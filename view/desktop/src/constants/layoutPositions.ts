import { ActivitybarPosition, SidebarPosition } from "@repo/moss-workspace";

// ActivityBar Position Constants
export const ACTIVITYBAR_POSITION: Record<ActivitybarPosition, ActivitybarPosition> = {
  DEFAULT: "DEFAULT",
  TOP: "TOP",
  BOTTOM: "BOTTOM",
  HIDDEN: "HIDDEN",
} as const;

// Sidebar Position Constants
export const SIDEBAR_POSITION: Record<SidebarPosition, SidebarPosition> = {
  LEFT: "LEFT",
  RIGHT: "RIGHT",
} as const;
