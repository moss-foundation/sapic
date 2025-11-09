import { Orientation, SerializedDockview } from "moss-tabs";

import { LayoutOutput } from "@/hooks/sharedStorage/layout/types";

export enum ACTIVITYBAR_POSITION {
  DEFAULT = "DEFAULT",
  TOP = "TOP",
  BOTTOM = "BOTTOM",
  HIDDEN = "HIDDEN",
}

export enum SIDEBAR_POSITION {
  LEFT = "LEFT",
  RIGHT = "RIGHT",
}

export const defaultSidebarPanel = {
  position: SIDEBAR_POSITION.LEFT,
  width: 255,
  visible: true,
  minWidth: 100,
  maxWidth: 400,
} as const;

export const defaultBottomPanePanel = {
  height: 333,
  minHeight: 100,
  maxHeight: Infinity,
  visible: false,
} as const;

export const emptyGridState: SerializedDockview = {
  grid: {
    root: {
      type: "branch",
      data: [],
    },
    height: 0,
    width: 0,
    orientation: Orientation.HORIZONTAL,
  },
  panels: {},
  activeGroup: undefined,
  floatingGroups: [],
  popoutGroups: [],
} as const;

export const defaultLayout: LayoutOutput = {
  sidebarState: defaultSidebarPanel,
  bottomPanelState: defaultBottomPanePanel,
  tabbedPaneState: {
    gridState: emptyGridState,
  },
  activitybarState: {
    position: ACTIVITYBAR_POSITION.DEFAULT,
    activeContainerId: "",
  },
};
