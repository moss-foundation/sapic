import { create } from "zustand";

interface WorkspaceStoreProps {
  workspace: string | null;
  setWorkspace: (workspace: string) => void;
}

export const useWorkspaceStore = create<WorkspaceStoreProps>((set) => ({
  workspace: null,
  setWorkspace: (workspace: string) => {
    set({ workspace });
  },
}));
