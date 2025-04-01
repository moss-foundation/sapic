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
      "id": "collections.groupId",
      "title": "Collections",
      "order": 1,
      "icon": "ActivityBarCollectionsIcon",
    },
    {
      "id": "environments.groupId",
      "title": "Environments",
      "order": 2,
      "icon": "ActivityBarEnvironmentsIcon",
    },
    {
      "id": "mock.groupId",
      "title": "Mock Servers",
      "order": 3,
      "icon": "ActivityBarMockIcon",
    },
  ],
};

export const MockGroupViews = {
  "collections.groupId": {
    "id": "collections",
    "name": "My View1",
    "component": "CollectionsList",
  },
  "environments.groupId": {
    "id": "environments",
    "name": "My View2",
    "component": "EnvironmentsList",
  },
  "mock.groupId": {
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
