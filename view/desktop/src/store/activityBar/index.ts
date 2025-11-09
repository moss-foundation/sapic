import { ComponentPropsWithoutRef } from "react";
import { create } from "zustand";

import { IconInlineType } from "@/components/IconInline";
import { Icons } from "@/lib/ui/Icon";
import {
  ActivitybarPartStateInfo,
  ActivitybarPosition,
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
  position: ActivitybarPosition;
  lastActiveContainerId: string | undefined;
  setPosition: (position: ActivitybarPosition) => void;
  setItems: (items: ActivityBarItemProps[]) => void;
  updateFromWorkspaceState: (activitybarState: ActivitybarPartStateInfo) => void;
  toWorkspaceState: () => ActivitybarPartStateInfo;
  resetToDefaults: () => void;
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

export const useActivityBarStore = create<ActivityBarStore>((set, get) => ({
  items: defaultItems,
  position: "DEFAULT",
  lastActiveContainerId: TREE_VIEW_GROUP_PROJECTS,
  setPosition: (position: ActivitybarPosition) => {
    set({ position });
  },
  setItems: (items: ActivityBarItemProps[]) => set({ items }),
  resetToDefaults: () => {
    set({
      items: [...defaultItems],
      position: "DEFAULT",
      lastActiveContainerId: TREE_VIEW_GROUP_PROJECTS,
    });
  },
  toWorkspaceState: (): ActivitybarPartStateInfo => {
    const state = get();

    return {
      lastActiveContainerId: state.lastActiveContainerId,
      position: state.position,
      items: state.items.map((item) => ({
        id: item.id,
        order: item.order,
        visible: item.isVisible !== false,
      })),
    };
  },
  updateFromWorkspaceState: (activitybarState: ActivitybarPartStateInfo) => {
    const currentItems = get().items;

    // Ensure we have a valid lastActiveContainerId, default to Projects if not
    const activeContainerId = activitybarState.lastActiveContainerId || TREE_VIEW_GROUP_PROJECTS;

    // Create a map of workspace state items by id for easy lookup
    const workspaceItemsMap = new Map(activitybarState.items.map((item) => [item.id, item]));

    // Update items with workspace state while preserving static properties
    const updatedItems = currentItems.map((item) => {
      const workspaceItem = workspaceItemsMap.get(item.id);
      if (workspaceItem) {
        return {
          ...item,
          order: workspaceItem.order,
          isVisible: workspaceItem.visible,
          isActive: item.id === activeContainerId,
        };
      }
      return {
        ...item,
        isActive: item.id === activeContainerId,
      };
    });

    // Sort items by order
    updatedItems.sort((a, b) => a.order - b.order);

    set({
      items: updatedItems,
      position: activitybarState.position,
      lastActiveContainerId: activeContainerId,
    });
  },
}));
