import { create } from "zustand";

import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface WorkspaceListStore {
  activeEnvironment: StreamEnvironmentsEvent | null;
  setActiveEnvironment: (environment: StreamEnvironmentsEvent) => void;
}

export const useWorkspaceListStore = create<WorkspaceListStore>((set) => ({
  activeEnvironment: null,
  setActiveEnvironment: (environment) => set({ activeEnvironment: environment }),
}));
