import { create } from "zustand";

import { WorkspaceMode } from "@repo/moss-workspace";

interface RequestModeStore {
  displayMode: WorkspaceMode;
  setDisplayMode: (displayMode: WorkspaceMode) => void;
  toggleDisplayMode: () => void;
}

export const useRequestModeStore = create<RequestModeStore>((set) => ({
  displayMode: "REQUEST_FIRST",
  setDisplayMode: (displayMode: WorkspaceMode) => {
    set({ displayMode });
  },
  toggleDisplayMode: () => {
    set((state) => ({
      displayMode: state.displayMode === "REQUEST_FIRST" ? "DESIGN_FIRST" : "REQUEST_FIRST",
    }));
  },
}));
