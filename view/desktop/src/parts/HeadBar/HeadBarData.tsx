import { type Icons } from "@/components/Icon";
import { type MenuItemProps } from "@/components/ActionMenu/ActionMenu";

/**
 * Helper function to generate standard menu items with unique IDs
 * @param prefix A prefix to add to item IDs to ensure uniqueness
 * @returns An array of MenuItemProps
 */
const createStandardMenuItems = (prefix = ""): MenuItemProps[] => {
  const idPrefix = prefix ? `${prefix}-` : "";

  return [
    {
      id: `${idPrefix}rename`,
      type: "action",
      label: "Rename...",
      icon: "ActionMenuRename" as Icons,
      shortcut: "⌘⏎",
    },
    {
      id: `${idPrefix}duplicate`,
      type: "action",
      label: "Duplicate",
      icon: "ActionMenuDuplicate" as Icons,
      shortcut: "⌘V",
    },
    {
      id: `${idPrefix}delete`,
      type: "action",
      label: "Delete",
      icon: "ActionMenuDelete" as Icons,
      shortcut: "⌥⇧⏎",
    },
    {
      id: `${idPrefix}separator-1`,
      type: "separator",
    },
    {
      id: `${idPrefix}new`,
      type: "submenu",
      label: "New...",
      icon: "PlusButton" as Icons,
      items: [
        {
          id: `${idPrefix}new-request`,
          type: "action",
          label: "Request",
        },
        {
          id: `${idPrefix}new-collection`,
          type: "action",
          label: "Collection",
        },
      ],
    },
    {
      id: `${idPrefix}separator-2`,
      type: "separator",
    },
    {
      id: `${idPrefix}save`,
      type: "action",
      label: "Save",
      shortcut: "⌘V",
      alignWithIcons: true,
    },
    {
      id: `${idPrefix}save-all`,
      type: "action",
      label: "Save All",
      shortcut: "⇧⌘8",
      alignWithIcons: true,
    },
    {
      id: `${idPrefix}separator-3`,
      type: "separator",
    },
    {
      id: `${idPrefix}edit-configurations`,
      type: "action",
      label: "Edit Configurations...",
      shortcut: "^⌥E",
      alignWithIcons: true,
    },
  ];
};

export const collectionActionMenuItems: MenuItemProps[] = createStandardMenuItems();

export const workspaceMenuItems: MenuItemProps[] = [
  {
    id: "new-workspace",
    type: "action",
    label: "New Workspace",
    icon: "PlusButton" as Icons,
  },
  ...createStandardMenuItems().slice(0, 4), // Include only the first 4 items (rename, duplicate, delete, separator)
  {
    id: "new-collection",
    type: "action",
    label: "New Collection",
    icon: "ActionMenuNewCollection" as Icons,
  },
  {
    id: "import-collection",
    type: "action",
    label: "Import Collection",
    icon: "ActionMenuImportCollection" as Icons,
  },
  {
    id: "separator-2",
    type: "separator",
  },
  {
    id: "save",
    type: "action",
    label: "Save",
    shortcut: "⌘V",
    alignWithIcons: true,
  },
  {
    id: "save-all",
    type: "action",
    label: "Save All",
    shortcut: "⇧⌘8",
    alignWithIcons: true,
  },
  {
    id: "separator-3",
    type: "separator",
  },
  {
    id: "all-workspaces",
    type: "accordion",
    label: "All Workspaces",
    icon: "TreeChevronRight",
    items: [
      {
        id: "microservices-api-test-suite",
        type: "submenu",
        label: "Microservices API Test Suite long name",
        icon: "ActionMenuWorkspace" as Icons,
        items: createStandardMenuItems("microservices-api"),
      },
      {
        id: "user-management-api",
        type: "submenu",
        label: "User Management API",
        icon: "ActionMenuWorkspace" as Icons,
        items: createStandardMenuItems("user-management"),
      },
      {
        id: "auth-security-tests",
        type: "submenu",
        label: "Auth & Security Tests",
        icon: "ActionMenuWorkspace" as Icons,
        items: createStandardMenuItems("auth-security"),
      },
      {
        id: "development-api-sandbox",
        type: "submenu",
        label: "Development API Sandbox",
        icon: "ActionMenuWorkspace" as Icons,
        items: createStandardMenuItems("dev-sandbox"),
      },
      {
        id: "microservices-endpoints",
        type: "submenu",
        label: "Microservices Endpoints",
        icon: "ActionMenuWorkspace" as Icons,
        items: createStandardMenuItems("micro-endpoints"),
      },
    ],
  },
  {
    id: "separator-4",
    type: "separator",
  },
  {
    id: "home",
    type: "action",
    label: "Kitchensink",
    icon: "TestHeadBarHome" as Icons,
  },
  {
    id: "logs",
    type: "action",
    label: "Logs",
    icon: "TestHeadBarLogs" as Icons,
  },
  {
    id: "debug",
    type: "action",
    label: "Debug Panels",
    icon: "TestHeadBarDebug" as Icons,
  },
  {
    id: "separator-5",
    type: "separator",
  },
  {
    id: "edit-configurations",
    type: "action",
    label: "Edit Configurations...",
    shortcut: "^⌥E",
    alignWithIcons: true,
  },
];
