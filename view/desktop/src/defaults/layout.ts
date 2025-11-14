import { Orientation, SerializedDockview } from "moss-tabs";

import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layout";
import { LayoutStateOutput } from "@/workbench/domains/layout/service";

export const defaultSidebarPanelState = {
  position: SIDEBAR_POSITION.LEFT,
  width: 255,
  visible: true,
  minWidth: 100,
  maxWidth: 400,
} as const;

export const defaultBottomPanePanelState = {
  height: 333,
  minHeight: 100,
  maxHeight: Infinity,
  visible: false,
} as const;

export const defaultActivityBarState = {
  position: ACTIVITYBAR_POSITION.DEFAULT,
  activeContainerId: "workbench.view.projects",
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

export const defaultLayoutState: LayoutStateOutput = {
  sidebarState: defaultSidebarPanelState,
  bottomPanelState: defaultBottomPanePanelState,
  tabbedPaneState: {
    gridState: emptyGridState,
  },
  activitybarState: defaultActivityBarState,
};
