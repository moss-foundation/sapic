import { SerializedDockview } from "moss-tabs";

import { ACTIVITYBAR_POSITION } from "./index";

export interface SidebarState {
  width: number;
  visible: boolean;
  minWidth: number;
  maxWidth: number;
}

export interface BottomPanelState {
  height: number;
  visible: boolean;
  minHeight: number;
  maxHeight: number;
}

export interface TabbedPaneState {
  gridState: SerializedDockview;
}

export interface ActivitybarState {
  position: ACTIVITYBAR_POSITION;
  activeContainerId: string;
}
