import { create } from "zustand";

import { AddAccountParams } from "@repo/moss-app";

interface GitProviderStore {
  gitProvider: AddAccountParams | null;
  setGitProvider: (gitProvider: AddAccountParams) => void;
}

export const useGitProviderStore = create<GitProviderStore>((set) => ({
  gitProvider: null,
  setGitProvider: (gitProvider) => set({ gitProvider }),
}));
