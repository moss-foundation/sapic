import { create } from "zustand";

import { invokeTauriIpc } from "@/lib/backend/tauri";
import { AddPanelOptions, DockviewApi, SerializedDockview } from "@repo/moss-tabs";

interface AddPanelOptionsWithoutMandatoryComponent extends Omit<AddPanelOptions, "component"> {
  component?: string;
}

interface TabbedPaneState {
  gridState: SerializedDockview;
  setGridState: (state: SerializedDockview) => void;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  api?: DockviewApi;
  setApi: (api: DockviewApi) => void;
  activePanelId: string | undefined;
  setActivePanelId: (id: string | undefined) => void;
  addOrFocusPanel: (options: AddPanelOptionsWithoutMandatoryComponent) => void;
}

export const useTabbedPaneStore = create<TabbedPaneState>((set, get) => ({
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
  setGridState: (state: SerializedDockview) => {
    invokeTauriIpc("set_layout_parts_state", {
      input: { editor: state },
      params: { isOnExit: false },
    });
    set({ gridState: state });
  },
  showDebugPanels: false,
  setShowDebugPanels: (show: boolean) => set({ showDebugPanels: show }),
  api: undefined,
  setApi: (api: DockviewApi) => set({ api }),
  activePanelId: undefined,
  setActivePanelId: (id: string | undefined) => set({ activePanelId: id }),
  addOrFocusPanel: async (options) => {
    const someRandomString = await new Promise<string>((resolve) => {
      setTimeout(() => {
        resolve(Math.random().toString(36).substring(7));
      }, 50);
    });

    const activePanel = get().api?.getPanel(options.id);

    if (activePanel) {
      activePanel.focus();
      return;
    }

    get().api?.addPanel({
      ...options,
      component: "Default",
      params: {
        ...options.params,
        someRandomString,
      },
    } as AddPanelOptions);
  },
}));
