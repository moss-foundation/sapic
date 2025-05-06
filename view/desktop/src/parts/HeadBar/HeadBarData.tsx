import { type MenuItemProps } from "@/components/ActionMenu/ActionMenu";
import { type Icons } from "@/lib/ui/Icon";
import { ListWorkspacesOutput } from "@repo/moss-workspace";

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
      icon: "Edit" as Icons,
      shortcut: "⌘⏎",
    },
    {
      id: `${idPrefix}duplicate`,
      type: "action",
      label: "Duplicate",
      icon: "ToolWindowDuplicates" as Icons,
      shortcut: "⌘V",
    },
    {
      id: `${idPrefix}delete`,
      type: "action",
      label: "Delete",
      icon: "Delete" as Icons,
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
      icon: "Add" as Icons,
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

/**
 * Creates the "All Workspaces" menu section with real workspace data
 * @param workspaces List of workspaces from the backend
 * @returns MenuItemProps for the all workspaces accordion section
 */
export const createAllWorkspacesMenuSection = (workspaces: ListWorkspacesOutput = []): MenuItemProps => {
  return {
    id: "all-workspaces",
    type: "accordion",
    label: "All Workspaces",
    icon: "ChevronRight",
    items: workspaces.map((workspace) => ({
      id: `workspace:${workspace.name}`,
      type: "submenu",
      label: workspace.name,
      icon: "WorkspaceActive" as Icons,
      items: createStandardMenuItems(workspace.name),
    })),
  };
};

// Base workspace menu items without dynamic workspaces
export const baseWorkspaceMenuItems: MenuItemProps[] = [
  {
    id: "new-workspace",
    type: "action",
    label: "New Workspace",
    icon: "NewFolder" as Icons,
  },
  {
    id: "open-workspace",
    type: "action",
    label: "Open Workspace",
    icon: "Folder" as Icons,
  },
  {
    id: "separator-1",
    type: "separator",
  },
];

// Only shown when a workspace is selected - base items without dynamic workspaces
export const baseSelectedWorkspaceMenuItems: MenuItemProps[] = [
  {
    id: "new-workspace",
    type: "action",
    label: "New Workspace",
    icon: "NewFolder" as Icons,
  },
  ...createStandardMenuItems().slice(0, 4), // Include only the first 4 items (rename, duplicate, delete, separator)
  {
    id: "new-collection",
    type: "action",
    label: "New Collection",
    icon: "Add" as Icons,
  },
  {
    id: "import-collection",
    type: "action",
    label: "Import Collection",
    icon: "Import" as Icons,
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
];

export const additionalSelectedWorkspaceMenuItems: MenuItemProps[] = [
  {
    id: "separator-4",
    type: "separator",
  },
  {
    id: "home",
    type: "action",
    label: "Kitchensink",
    alignWithIcons: true,
  },
  {
    id: "logs",
    type: "action",
    label: "Logs",
    alignWithIcons: true,
  },
  {
    id: "debug",
    type: "action",
    label: "Debug Panels",
    alignWithIcons: true,
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
