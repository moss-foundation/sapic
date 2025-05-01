import { type Icons } from "@/components/Icon";
import { type MenuItemProps } from "@/components/ActionMenu/ActionMenu";

export const collectionActionMenuItems: MenuItemProps[] = [
  {
    id: "rename",
    type: "action",
    label: "Rename...",
    icon: "ActionMenuRename" as Icons,
    shortcut: "⌘⏎",
  },
  {
    id: "duplicate",
    type: "action",
    label: "Duplicate",
    icon: "ActionMenuDuplicate" as Icons,
    shortcut: "⌘V",
  },
  {
    id: "delete",
    type: "action",
    label: "Delete",
    icon: "ActionMenuDelete" as Icons,
    shortcut: "⌥⇧⏎",
  },
  {
    id: "separator-1",
    type: "separator",
  },
  {
    id: "new",
    type: "submenu",
    label: "New...",
    icon: "PlusButton" as Icons,
    items: [
      {
        id: "new-request",
        type: "action",
        label: "Request",
      },
      {
        id: "new-collection",
        type: "action",
        label: "Collection",
      },
    ],
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
    id: "edit-configurations",
    type: "action",
    label: "Edit Configurations...",
    shortcut: "^⌥E",
    alignWithIcons: true,
  },
];

// Workspace menu items based on the screenshot
export const workspaceMenuItems: MenuItemProps[] = [
  {
    id: "new-workspace",
    type: "action",
    label: "New Workspace",
    icon: "PlusButton" as Icons,
  },
  {
    id: "rename",
    type: "action",
    label: "Rename...",
    icon: "ActionMenuRename" as Icons,
    shortcut: "⌘⏎",
  },
  {
    id: "duplicate",
    type: "action",
    label: "Duplicate",
    icon: "ActionMenuDuplicate" as Icons,
    shortcut: "⌘V",
  },
  {
    id: "delete",
    type: "action",
    label: "Delete",
    icon: "ActionMenuDelete" as Icons,
    shortcut: "⌥⇧⏎",
  },
  {
    id: "separator-1",
    type: "separator",
  },
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
    type: "action",
    label: "All Workspaces",
    icon: "TreeChevronRight",
    count: 5,
  },
  {
    id: "separator-4",
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
