import { create } from "zustand";

export type SideBarPosition = "left" | "right";

interface SideBarState {
  sideBarPosition: SideBarPosition;
  setSideBarPosition: (position: SideBarPosition) => void;
}

export const useSideBarStore = create<SideBarState>((set) => ({
  sideBarPosition: "left",
  setSideBarPosition: (position: SideBarPosition) => set({ sideBarPosition: position }),
}));
