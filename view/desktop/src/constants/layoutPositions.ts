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
  size: 255,
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
