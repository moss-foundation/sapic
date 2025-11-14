import { SerializedDockview } from "moss-tabs";

import { ActivitybarPosition } from "@repo/moss-workspace";

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
  position: ActivitybarPosition;
  activeContainerId: string;
}
