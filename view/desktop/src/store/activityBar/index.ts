import { create } from "zustand";

import { Icons } from "@/lib/ui/Icon";
import {
  ActivitybarPartStateInfo,
  ActivitybarPosition,
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
  TREE_VIEW_GROUP_PROJECTS,
} from "@repo/moss-workspace";

export interface ActivityBarItem {
  id: string;
  icon: Icons;
  iconActive: Icons;
  title: string;
  order: number;
  isActive: boolean;
  isVisible?: boolean;
}

export interface ActivityBarStore {
  items: ActivityBarItem[];
  position: ActivitybarPosition;
  lastActiveContainerId: string | undefined;
  setPosition: (position: ActivitybarPosition) => void;
  setItems: (items: ActivityBarItem[]) => void;
  getActiveItem: () => ActivityBarItem | undefined;
  updateFromWorkspaceState: (activitybarState: ActivitybarPartStateInfo) => void;
  setActiveItem: (itemId: string) => void;
  toWorkspaceState: () => ActivitybarPartStateInfo;
  resetToDefaults: () => void;
}

const defaultItems: ActivityBarItem[] = [
  {
    "id": TREE_VIEW_GROUP_PROJECTS,
    "title": "Collections",
    "order": 1,
    "icon": "Home",
    "iconActive": "HomeActive",
    "isActive": true,
    "isVisible": true,
  },
  {
    "id": TREE_VIEW_GROUP_ENVIRONMENTS,
    "title": "Environments",
    "order": 2,
    "icon": "JsonPath",
    "iconActive": "JsonPathActive",
    "isActive": false,
    "isVisible": true,
  },
  {
    "id": TREE_VIEW_GROUP_MOCK_SERVERS,
    "title": "Mock",
    "order": 3,
    "icon": "WebServer",
    "iconActive": "WebServerActive",
    "isActive": false,
    "isVisible": true,
  },
  {
    "id": "4",
    "title": "Preferences",
    "order": 4,
    "icon": "Wrench",
    "iconActive": "WrenchActive",
    "isActive": false,
    "isVisible": true,
  },
  {
    "id": "5",
    "title": "Commit",
    "order": 5,
    "icon": "Commit",
    "iconActive": "CommitActive",
    "isActive": false,
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
  setItems: (items: ActivityBarItem[]) => set({ items }),
  getActiveItem: () => {
    return get().items.find((item) => item.isActive);
  },
  setActiveItem: (itemId: string) => {
    const currentItems = get().items;
    const updatedItems = currentItems.map((item) => ({
      ...item,
      isActive: item.id === itemId,
    }));
    set({ items: updatedItems, lastActiveContainerId: itemId });
  },
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

    // Ensure we have a valid lastActiveContainerId, default to Collections if not
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
