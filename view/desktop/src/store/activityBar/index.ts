import { create } from "zustand";

import { Icons } from "@/lib/ui/Icon";
import {
  ActivitybarPartStateInfo,
  ActivitybarPosition,
  TREE_VIEW_GROUP_COLLECTIONS,
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
} from "@repo/moss-workspace";

export interface ActivityBarItem {
  id: string;
  icon: Icons;
  iconActive: Icons;
  title: string;
  order: number;
  isActive: boolean;
  visible?: boolean;
}

interface ActivityBarStore {
  items: ActivityBarItem[];
  position: ActivitybarPosition;
  lastActiveContainerId: string | null;
  setPosition: (position: ActivitybarPosition) => void;
  setItems: (items: ActivityBarItem[]) => void;
  getActiveItem: () => ActivityBarItem | undefined;
  updateFromWorkspaceState: (activitybarState: ActivitybarPartStateInfo) => void;
  setActiveItem: (itemId: string) => void;
  toWorkspaceState: () => ActivitybarPartStateInfo;
  resetToDefaults: () => void;
}

// Default activity bar items with static properties
const defaultItems: ActivityBarItem[] = [
  {
    "id": TREE_VIEW_GROUP_COLLECTIONS,
    "title": "Collections",
    "order": 1,
    "icon": "Folder",
    "iconActive": "FolderActive",
    "isActive": true,
    "visible": true,
  },
  {
    "id": TREE_VIEW_GROUP_ENVIRONMENTS,
    "title": "Environments",
    "order": 2,
    "icon": "Env",
    "iconActive": "EnvActive",
    "isActive": false,
    "visible": true,
  },
  {
    "id": TREE_VIEW_GROUP_MOCK_SERVERS,
    "title": "Mock",
    "order": 3,
    "icon": "WebServer",
    "iconActive": "WebServerActive",
    "isActive": false,
    "visible": true,
  },
];

export const useActivityBarStore = create<ActivityBarStore>((set, get) => ({
  items: defaultItems,
  position: "DEFAULT",
  lastActiveContainerId: TREE_VIEW_GROUP_COLLECTIONS,
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
      lastActiveContainerId: TREE_VIEW_GROUP_COLLECTIONS,
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
        visible: item.visible !== false,
      })),
    };
  },
  updateFromWorkspaceState: (activitybarState: ActivitybarPartStateInfo) => {
    const currentItems = get().items;

    // Ensure we have a valid lastActiveContainerId, default to Collections if not
    const activeContainerId = activitybarState.lastActiveContainerId || TREE_VIEW_GROUP_COLLECTIONS;

    // Create a map of workspace state items by id for easy lookup
    const workspaceItemsMap = new Map(activitybarState.items.map((item) => [item.id, item]));

    // Update items with workspace state while preserving static properties
    const updatedItems = currentItems.map((item) => {
      const workspaceItem = workspaceItemsMap.get(item.id);
      if (workspaceItem) {
        return {
          ...item,
          order: workspaceItem.order,
          visible: workspaceItem.visible,
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
