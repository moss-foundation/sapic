import {
  TREE_VIEW_GROUP_COLLECTIONS,
  TREE_VIEW_GROUP_ENVIRONMENTS,
  TREE_VIEW_GROUP_MOCK_SERVERS,
} from "@repo/moss-workspace";

// ViewGroups mock data
export interface ViewGroup {
  id: string;
  title: string;
  order: number;
  icon: string;
}

export interface Views {
  viewGroups: ViewGroup[];
}

export interface GroupView {
  id: string;
  name: string;
  component: string;
}

export const MockViews: Views = {
  "viewGroups": [
    {
      "id": TREE_VIEW_GROUP_COLLECTIONS,
      "title": "Collections",
      "order": 1,
      "icon": "ActivityBarCollections",
    },
    {
      "id": TREE_VIEW_GROUP_ENVIRONMENTS,
      "title": "Environments",
      "order": 2,
      "icon": "ActivityBarEnvironments",
    },
    {
      "id": TREE_VIEW_GROUP_MOCK_SERVERS,
      "title": "Mock Servers",
      "order": 3,
      "icon": "ActivityBarMock",
    },
  ],
};

export const MockGroupViews = {
  [TREE_VIEW_GROUP_COLLECTIONS]: {
    "id": "collections",
    "name": "My View1",
    "component": "CollectionsList",
  },
  [TREE_VIEW_GROUP_ENVIRONMENTS]: {
    "id": "environments",
    "name": "My View2",
    "component": "EnvironmentsList",
  },
  [TREE_VIEW_GROUP_MOCK_SERVERS]: {
    "id": "mock",
    "name": "My View3",
    "component": "MockServersList",
  },
};

// ActivityBarState mock data
export type ActivityBarPosition = "top" | "bottom" | "hidden" | "default";

export interface ActivityBarState {
  position: ActivityBarPosition;
  groupOrder: string[];
}

export const MockActivityBarState: ActivityBarState = {
  position: "default",
  groupOrder: [],
};

// AppLayoutState mock data
export interface AppLayoutState {
  activeSidebar: "left" | "right" | "none";
  sidebarSetting: "left" | "right";
}

export const MockAppLayoutState: AppLayoutState = {
  activeSidebar: "left",
  sidebarSetting: "left",
};
