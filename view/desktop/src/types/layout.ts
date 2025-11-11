import { SerializedDockview } from "moss-tabs";

import { ActivitybarPosition } from "@repo/moss-workspace";

export interface LayoutStateOutput {
  sidebarState: {
    width: number;
    visible: boolean;
    minWidth: number;
    maxWidth: number;
  };
  bottomPanelState: {
    height: number;
    visible: boolean;
    minHeight: number;
    maxHeight: number;
  };
  tabbedPaneState: {
    gridState: SerializedDockview;
  };
  activitybarState: {
    position: ActivitybarPosition;
    activeContainerId: string;
  };
}

type DeepPartial<T> = T extends object ? { [K in keyof T]?: DeepPartial<T[K]> } : T;
type Simplify<T> = { [K in keyof T]: T[K] } & {};

export type LayoutStateInput = Simplify<DeepPartial<LayoutStateOutput>>;
