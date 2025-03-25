import { create } from "zustand";

export type ActivityBarPosition = "left" | "right" | "top" | "bottom" | "hidden" | "default";

interface ActivityBarState {
  activityBarPosition: ActivityBarPosition;
  setActivityBarPosition: (position: ActivityBarPosition) => void;
}

export const useActivityBarStore = create<ActivityBarState>((set) => ({
  activityBarPosition: "left",
  setActivityBarPosition: (position: ActivityBarPosition) => set({ activityBarPosition: position }),
}));
