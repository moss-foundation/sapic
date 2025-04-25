import { create } from "zustand";

interface WorkspaceStoreProps {
  workspace: string | null;
  setWorkspace: (workspace: string) => void;
}

//FIXME this whole store should be removed, because workspaces should be handled through TanStack Query, it's a temporary solution
export const useWorkspaceStore = create<WorkspaceStoreProps>((set) => ({
  workspace: null,
  setWorkspace: (workspace: string) => {
    set({ workspace });
  },
}));
