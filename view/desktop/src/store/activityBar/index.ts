import { create } from "zustand";

import { Icons } from "@/components";

export interface ActivityBarItem {
  id: string;
  icon: Icons;
  title: string;
  order: number;
  isActive: boolean;
  isFill?: boolean;
  isStroke?: boolean;
}

interface ActivityBarStore {
  items: ActivityBarItem[];
  alignment: "vertical" | "horizontal";
  position: "default" | "top" | "bottom" | "hidden";
  setPosition: (position: ActivityBarStore["position"]) => void;
  setItems: (items: ActivityBarItem[]) => void;
  setAlignment: (alignment: ActivityBarStore["alignment"]) => void;
  getActiveItem: () => ActivityBarItem | undefined;
}

export const useActivityBarStore = create<ActivityBarStore>((set, get) => ({
  items: [
    {
      "id": "collections.groupId",
      "title": "Collections",
      "order": 1,
      "icon": "ActivityBarCollectionsTest",
      "isActive": true,
      "isFill": true,
      "isStroke": true,
    },
    {
      "id": "environments.groupId",
      "title": "Environments",
      "order": 2,
      "icon": "ActivityBarEnvironmentsTest",
      "isActive": false,
      "isFill": false,
      "isStroke": true,
    },
    {
      "id": "mock.groupId",
      "title": "Mock",
      "order": 3,
      "icon": "ActivityBarMockTest",
      "isActive": false,
      "isFill": true,
      "isStroke": true,
    },
  ],
  alignment: "vertical",
  position: "default",
  setPosition: (position: ActivityBarStore["position"]) => {
    if (position === "default") {
      set({ position, alignment: "vertical" });
    }
    if (position === "top" || position === "bottom") {
      set({ position, alignment: "horizontal" });
    }
    if (position === "hidden") {
      set({ position });
    }
  },
  setItems: (items: ActivityBarItem[]) => set({ items }),
  setAlignment: (alignment: ActivityBarStore["alignment"]) => set({ alignment }),
  getActiveItem: () => {
    return get().items.find((item) => item.isActive);
  },
}));
