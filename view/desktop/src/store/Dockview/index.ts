import { create } from "zustand";

import { AddPanelOptions, DockviewApi } from "@repo/moss-tabs";

interface AddPanelOptionsWithoutMandatoryComponent extends Omit<AddPanelOptions, "component"> {
  component?: string;
}

interface DockviewApiState {
  api: DockviewApi | undefined;
  currentActivePanelId: string | undefined;
  setCurrentActivePanelId: (id: string | undefined) => void;
  addPanel: (options: AddPanelOptionsWithoutMandatoryComponent) => void;
  setApi: (api: DockviewApi) => void;
}

export const useDockviewStore = create<DockviewApiState>((set, get) => ({
  api: undefined,
  currentActivePanelId: undefined,
  addPanel: async (options) => {
    const someRandomString = await new Promise<string>((resolve) => {
      setTimeout(() => {
        resolve(Math.random().toString(36).substring(7));
      }, 50);
    });

    get().api?.addPanel({
      ...options,
      component: "Default",
      params: {
        someRandomString,
      },
    } as AddPanelOptions);
  },
  setApi: (api: DockviewApi) => {
    set({ api });
  },
  setCurrentActivePanelId: (id: string | undefined) => {
    if (id === undefined) return;
    set({ currentActivePanelId: id });
  },
}));
