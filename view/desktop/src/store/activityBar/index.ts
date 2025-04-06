import { create } from "zustand";

import { Icons } from "@/components";

export interface ActivityBarItem {
  id: string;
  icon: Icons;
  order: number;
  isActive: boolean;
}

interface ActivityBarStore {
  items: ActivityBarItem[];
  alignment: "vertical" | "horizontal";
  position: "default" | "top" | "bottom" | "hidden";
  setPosition: (position: ActivityBarStore["position"]) => void;
  setItems: (items: ActivityBarItem[]) => void;
}

export const useActivityBarStore = create<ActivityBarStore>((set) => ({
  items: [
    {
      "id": "collections.groupId",
      "order": 1,
      "icon": "ActivityBarCollectionsIcon",
      "isActive": true,
    },
    {
      "id": "environments.groupId",
      "order": 2,
      "icon": "ActivityBarEnvironmentsIcon",
      "isActive": false,
    },
    {
      "id": "mock.groupId",
      order: 3,
      icon: "ActivityBarMockIcon",
      isActive: false,
    },
  ],
  alignment: "vertical",
  position: "default",
  setPosition: (position: ActivityBarStore["position"]) => set({ position }),
  setItems: (items: ActivityBarItem[]) => set({ items }),
}));
