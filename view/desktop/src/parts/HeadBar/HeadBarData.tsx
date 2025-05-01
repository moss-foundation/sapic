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
        id: "new-file",
        type: "action",
        label: "File",
      },
      {
        id: "new-folder",
        type: "action",
        label: "Folder",
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
