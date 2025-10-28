import { create } from "zustand";

import { WorkspaceMode } from "@repo/moss-workspace";

interface WorkspaceModeStore {
  displayMode: WorkspaceMode;
  setDisplayMode: (displayMode: WorkspaceMode) => void;
  toggleDisplayMode: () => void;
}

export const useWorkspaceModeStore = create<WorkspaceModeStore>((set) => ({
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
