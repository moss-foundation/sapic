import { create } from "zustand";

import { AddAccountOutput } from "@repo/moss-app";

interface GitProviderStore {
  gitProvider: AddAccountOutput | null;
  setGitProvider: (gitProvider: AddAccountOutput) => void;
}

export const useGitProviderStore = create<GitProviderStore>((set) => ({
  gitProvider: null,
  setGitProvider: (gitProvider) => set({ gitProvider }),
}));
