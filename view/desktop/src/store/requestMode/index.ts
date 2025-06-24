import { create } from "zustand";

interface RequestModeStore {
  displayMode: "RequestFirst" | "DesignFirst";
  setDisplayMode: (displayMode: "RequestFirst" | "DesignFirst") => void;
  toggleDisplayMode: () => void;
}

export const useRequestModeStore = create<RequestModeStore>((set) => ({
  displayMode: "RequestFirst",
  setDisplayMode: (displayMode: "RequestFirst" | "DesignFirst") => {
    set({ displayMode });
  },
  toggleDisplayMode: () => {
    set((state) => ({
      displayMode: state.displayMode === "RequestFirst" ? "DesignFirst" : "RequestFirst",
    }));
  },
}));
