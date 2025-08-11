import { create } from "zustand";

import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

//TODO this whole store should be removed. It is a placeholder for the workspace list. Remove it once the functionality for active environment is implemented.
interface WorkspaceListStore {
  activeEnvironment: StreamEnvironmentsEvent | null;
  setActiveEnvironment: (environment: StreamEnvironmentsEvent) => void;
}

export const useWorkspaceListStore = create<WorkspaceListStore>((set) => ({
  activeEnvironment: null,
  setActiveEnvironment: (environment) => set({ activeEnvironment: environment }),
}));
