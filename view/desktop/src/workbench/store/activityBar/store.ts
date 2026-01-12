import { create } from "zustand";

import { defaultItems } from "./defaults";
import type { ActivityBarStore } from "./types";

export const useActivityBarStore = create<ActivityBarStore>((set) => ({
  items: defaultItems,
  setItems: (items) => set({ items }),
}));
