import { AddPanelOptions, DockviewApi, SerializedDockview } from "moss-tabs";
import { create } from "zustand";

import { emptyGridState } from "@/workbench/domains/layout/defaults";

import { TypedAddPanelOptions } from "./types";

interface TabbedPaneState {
  gridState: SerializedDockview;
  setGridState: (state: SerializedDockview) => void;

  api?: DockviewApi;
  setApi: (api: DockviewApi) => void;

  activePanelId: string | undefined;
  setActivePanelId: (id: string | undefined) => void;

  addOrFocusPanel: (options: TypedAddPanelOptions) => void;
  openPanel: (panelType: string) => void;
  removePanel: (panelId: string) => void;

  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;

  watermark: boolean;
  setWatermark: (watermark: boolean) => void;
}

export const useTabbedPaneStore = create<TabbedPaneState>((set, get) => ({
  gridState: emptyGridState,
  setGridState: (state: SerializedDockview) => {
    set({ gridState: state });
  },

  api: undefined,
  setApi: (api: DockviewApi) => set({ api }),

  activePanelId: undefined,
  setActivePanelId: (id: string | undefined) => set({ activePanelId: id }),

  addOrFocusPanel: async (options) => {
    const activePanel = get().api?.getPanel(options.id);

    if (activePanel && !activePanel.api.isFocused) {
      activePanel.focus();
    } else {
      // We perform a double-cast (as unknown as AddPanelOptions) here because
      // 'options'(TypedAddPanelOptions) is a complex Discriminated Union (it is needed for complex TS type checking when calling addOrFocusPanel)
      // that TypeScript cannot automatically map to the generic 'AddPanelOptions' type expected by the library.
      get().api?.addPanel({
        ...options,
        component: options.component,
        params: {
          ...(options.params || {}),
        },
      } as unknown as AddPanelOptions);
    }
  },
  openPanel: (panelType: string) => {
    try {
      // Use setTimeout to prevent race condition during initialization
      setTimeout(() => {
        const api = get().api;
        if (!api) return;

        try {
          if (api.getPanel(panelType) !== undefined) {
            api.getPanel(panelType)?.focus();
            return;
          }
          api.addPanel({
            id: panelType,
            component: panelType,
            renderer: "onlyWhenVisible",
          });
        } catch (error) {
          console.error(`Error opening ${panelType} panel:`, error);
        }
      }, 0);
    } catch (error) {
      console.error(`Error in open${panelType}:`, error);
    }
  },
  removePanel: (panelId: string) => {
    const panel = get().api?.getPanel(panelId);
    if (panel) {
      get().api?.removePanel(panel);
    }
  },

  showDebugPanels: false,
  setShowDebugPanels: (show: boolean) => set({ showDebugPanels: show }),

  watermark: false,
  setWatermark: (watermark: boolean) => set({ watermark }),
}));
