import { create } from "zustand";

import { DockviewApi, SerializedDockview } from "@repo/moss-tabs";

interface DockviewApiState {
  api: DockviewApi | undefined;
  currentActivePanelId: string | undefined;
  setCurrentActivePanelId: (id: string | undefined) => void;
  addPanel: (id: string | number) => void;
  setApi: (api: DockviewApi) => void;
}

export const useDockviewStore = create<DockviewApiState>((set, get) => ({
  api: undefined,
  currentActivePanelId: undefined,
  addPanel: (id) => {
    get().api?.addPanel({
      id: String(id),
      component: "Default",
    });
  },
  setApi(api: DockviewApi) {
    set({ api });
  },
  setCurrentActivePanelId(id: string | undefined) {
    if (id === undefined) return;
    set({ currentActivePanelId: id });
  },
}));
