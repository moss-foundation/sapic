import { MenuItemProps } from "@/workbench/utils/renderActionMenuItem";
import { ListWorkspacesOutput } from "@repo/window";

const createStandardMenuItems = (prefix = ""): MenuItemProps[] => {
  const idPrefix = prefix ? `${prefix}-` : "";

  return [
    {
      id: `${idPrefix}rename`,
      type: "action",
      label: "Edit",
      icon: "Edit",
      shortcut: "⌘⏎",
    },
    {
      id: `${idPrefix}duplicate`,
      type: "action",
      label: "Duplicate",
      icon: "ToolWindowDuplicates",
      shortcut: "⌘V",
    },
    {
      id: `${idPrefix}delete`,
      type: "action",
      label: "Delete",
      icon: "Delete",
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
      icon: "Add",
      items: [
        {
          id: `${idPrefix}new-endpoint`,
          type: "action",
          label: "Endpoint",
        },
        {
          id: `${idPrefix}new-project`,
          type: "action",
          label: "Project",
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

export const projectActionMenuItems: MenuItemProps[] = createStandardMenuItems();

export const createAllWorkspacesMenuSection = (workspaces: ListWorkspacesOutput = []): MenuItemProps => {
  return {
    id: "all-workspaces",
    type: "accordion",
    label: "All Workspaces",
    icon: "ChevronRight",
    items: workspaces.map((workspace) => ({
      id: `workspace:${workspace.id}`,
      type: "submenu",
      label: workspace.name,
      icon: "Workspace",
      items: createStandardMenuItems(workspace.id),
    })),
  };
};

// Base workspace menu items without dynamic workspaces
export const baseWorkspaceMenuItems: MenuItemProps[] = [
  {
    id: "new-workspace",
    type: "action",
    label: "New Workspace",
    icon: "NewWorkspace",
  },
  {
    id: "open-workspace",
    type: "action",
    label: "Open Workspace",
    icon: "Workspace",
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
    icon: "NewWorkspace",
  },
  ...createStandardMenuItems().slice(0, 4), // Include only the first 4 items (rename, duplicate, delete, separator)
  {
    id: "new-project",
    type: "action",
    label: "New Project",
    icon: "Add",
  },
  {
    id: "import-project",
    type: "action",
    label: "Import Project",
    icon: "Import",
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
    id: "kitchensink",
    type: "action",
    label: "KitchenSink",
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
  {
    id: "separator-6",
    type: "separator",
  },
  {
    id: "exit-workspace",
    type: "action",
    label: "Exit Workspace",
    alignWithIcons: true,
  },
];
