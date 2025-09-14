import { create } from "zustand";

import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { Icons } from "@/lib/ui";
import { AddPanelOptions, DockviewApi, SerializedDockview } from "@repo/moss-tabs";

interface AddPanelOptionsWithoutMandatoryComponent
  extends Omit<
    AddPanelOptions<{
      iconType?: Icons;
      projectId?: string;
      node?: ProjectTreeNode;
      workspace?: boolean;
    }>,
    "component"
  > {
  component?: string;
}

interface TabbedPaneState {
  gridState: SerializedDockview;
  showDebugPanels: boolean;
  setShowDebugPanels: (show: boolean) => void;
  api?: DockviewApi;
  setApi: (api: DockviewApi) => void;
  activePanelId: string | undefined;
  setActivePanelId: (id: string | undefined) => void;
  addOrFocusPanel: (options: AddPanelOptionsWithoutMandatoryComponent) => void;
  setGridState: (state: SerializedDockview) => void;
  openPanel: (panelType: string) => void;
  removePanel: (panelId: string) => void;
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
    set({ gridState: state });
  },
  showDebugPanels: false,
  setShowDebugPanels: (show: boolean) => set({ showDebugPanels: show }),
  api: undefined,
  setApi: (api: DockviewApi) => set({ api }),
  activePanelId: undefined,
  setActivePanelId: (id: string | undefined) => set({ activePanelId: id }),
  addOrFocusPanel: async (options) => {
    const activePanel = get().api?.getPanel(options.id);

    if (activePanel) {
      activePanel.focus();
      return;
    }

    get().api?.addPanel({
      ...options,
      component: options.component || "Default",
      params: {
        ...options.params,
      },
    } as AddPanelOptions);
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
}));
