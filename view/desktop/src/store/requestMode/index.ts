import { create } from "zustand";

import { WorkspaceMode } from "@repo/moss-workspace";

interface RequestModeStore {
  displayMode: WorkspaceMode;
  setDisplayMode: (displayMode: WorkspaceMode) => void;
  toggleDisplayMode: () => void;
}

export const useRequestModeStore = create<RequestModeStore>((set) => ({
  displayMode: "LIVE",
  setDisplayMode: (displayMode: WorkspaceMode) => {
    set({ displayMode });
  },
  toggleDisplayMode: () => {
    set((state) => ({
      displayMode: state.displayMode === "LIVE" ? "DESIGN" : "LIVE",
    }));
  },
}));
