import { create } from "zustand";

import { DockviewApi, SerializedDockview } from "@repo/moss-tabs";

interface TabbedPaneState {
  gridState: SerializedDockview;
  setGridState: (state: SerializedDockview) => void;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  api?: DockviewApi;
  setApi: (api: DockviewApi) => void;
}

export const useTabbedPaneStore = create<TabbedPaneState>((set) => ({
  gridState: {
    grid: {
      root: {
        type: "branch",
        data: [],
      },
      height: 0,
      width: 0,
      orientation: "horizontal" as SerializedDockview["grid"]["orientation"],
    },
    panels: {},
    activeGroup: undefined,
    floatingGroups: [],
    popoutGroups: [],
  } as SerializedDockview,
  setGridState: (state: SerializedDockview) => set({ gridState: state }),
  showDebugPanels: false,
  setShowDebugPanels: (show: boolean) => set({ showDebugPanels: show }),
  api: undefined,
  setApi: (api: DockviewApi) => set({ api }),
}));
