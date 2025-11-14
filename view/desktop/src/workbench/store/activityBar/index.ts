import { ComponentPropsWithoutRef } from "react";
import { create } from "zustand";

import { Icons } from "@/lib/ui/Icon";
import { IconInlineType } from "@/workbench/ui/components/IconInline";
import {
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
  TREE_VIEW_GROUP_PROJECTS,
} from "@repo/moss-workspace";

export interface ActivityBarItemProps extends ComponentPropsWithoutRef<"button"> {
  id: string;
  icon: Icons;
  iconActive?: IconInlineType;
  title: string;
  order: number;
  isVisible?: boolean;
  isDraggable?: boolean;
}

export interface ActivityBarStore {
  items: ActivityBarItemProps[];
  setItems: (items: ActivityBarItemProps[]) => void;
}

const defaultItems: ActivityBarItemProps[] = [
  {
    "id": TREE_VIEW_GROUP_PROJECTS,
    "title": "Projects",
    "order": 1,
    "icon": "Home",
    "iconActive": "HomeActive",
    "isVisible": true,
  },
  {
    "id": TREE_VIEW_GROUP_ENVIRONMENTS,
    "title": "Environments",
    "order": 2,
    "icon": "JsonPath",
    "iconActive": "JsonPathActive",
    "isVisible": true,
  },
  {
    "id": TREE_VIEW_GROUP_MOCK_SERVERS,
    "title": "Mock",
    "order": 3,
    "icon": "WebServer",
    "iconActive": "WebServerActive",
    "isVisible": true,
  },
  {
    "id": "4",
    "title": "Preferences",
    "order": 4,
    "icon": "Wrench",
    "iconActive": "WrenchActive",
    "isVisible": true,
  },
  {
    "id": "5",
    "title": "Commit",
    "order": 5,
    "icon": "Commit",
    "iconActive": "CommitActive",
    "isVisible": true,
  },
];

export const useActivityBarStore = create<ActivityBarStore>((set) => ({
  items: defaultItems,
  setItems: (items: ActivityBarItemProps[]) => set({ items }),
}));
