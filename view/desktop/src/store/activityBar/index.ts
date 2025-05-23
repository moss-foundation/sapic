import { create } from "zustand";

import { Icons } from "@/lib/ui/Icon";

export interface ActivityBarItem {
  id: string;
  icon: Icons;
  iconActive: Icons;
  title: string;
  order: number;
  isActive: boolean;
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
      "icon": "Folder",
      "iconActive": "FolderActive",
      "isActive": true,
    },
    {
      "id": "environments.groupId",
      "title": "Environments",
      "order": 2,
      "icon": "Env",
      "iconActive": "EnvActive",
      "isActive": false,
    },
    {
      "id": "mock.groupId",
      "title": "Mock",
      "order": 3,
      "icon": "WebServer",
      "iconActive": "WebServerActive",
      "isActive": false,
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
