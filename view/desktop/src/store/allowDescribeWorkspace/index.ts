import { create } from "zustand";

interface AllowDescribeWorkspaceStore {
  allow: boolean;
  setAllow: (allow: boolean) => void;
}

export const useAllowDescribeWorkspaceStore = create<AllowDescribeWorkspaceStore>((set) => ({
  allow: false,
  setAllow: (allow: boolean) => set({ allow }),
}));
