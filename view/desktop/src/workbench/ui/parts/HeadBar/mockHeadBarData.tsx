import { MenuItemProps } from "@/workbench/utils/renderActionMenuItem";

// Windows menu items
export const windowsMenuItems: MenuItemProps[] = [
  {
    id: "file",
    type: "submenu",
    label: "File",
    alignWithIcons: true,
    items: [
      {
        id: "new-file",
        type: "action",
        label: "New File",
        shortcut: "Ctrl+N",
      },
      {
        id: "new-folder",
        type: "action",
        label: "New Folder",
        shortcut: "Ctrl+Shift+N",
      },
      {
        id: "open",
        type: "action",
        label: "Open...",
        shortcut: "Ctrl+O",
      },
      {
        id: "separator-1",
        type: "separator",
      },
      {
        id: "save",
        type: "action",
        label: "Save",
        shortcut: "Ctrl+S",
      },
      {
        id: "save-as",
        type: "action",
        label: "Save As...",
        shortcut: "Ctrl+Shift+S",
      },
      {
        id: "separator-2",
        type: "separator",
      },
      {
        id: "exit",
        type: "action",
        label: "Exit",
        shortcut: "Alt+F4",
      },
    ],
  },
  {
    id: "edit",
    type: "submenu",
    label: "Edit",
    alignWithIcons: true,
    items: [
      {
        id: "undo",
        type: "action",
        label: "Undo",
        shortcut: "Ctrl+Z",
      },
      {
        id: "redo",
        type: "action",
        label: "Redo",
        shortcut: "Ctrl+Y",
      },
      {
        id: "separator-3",
        type: "separator",
      },
      {
        id: "cut",
        type: "action",
        label: "Cut",
        shortcut: "Ctrl+X",
      },
      {
        id: "copy",
        type: "action",
        label: "Copy",
        shortcut: "Ctrl+C",
      },
      {
        id: "paste",
        type: "action",
        label: "Paste",
        shortcut: "Ctrl+V",
      },
      {
        id: "separator-4",
        type: "separator",
      },
      {
        id: "find",
        type: "action",
        label: "Find",
        shortcut: "Ctrl+F",
      },
      {
        id: "replace",
        type: "action",
        label: "Replace",
        shortcut: "Ctrl+H",
      },
    ],
  },
  {
    id: "view",
    type: "submenu",
    label: "View",
    alignWithIcons: true,
    items: [
      {
        id: "explorer",
        type: "checkable",
        label: "Explorer",
        checked: true,
      },
      {
        id: "search",
        type: "checkable",
        label: "Search",
        checked: true,
      },
      {
        id: "debug",
        type: "checkable",
        label: "Debug",
        checked: false,
      },
      {
        id: "separator-5",
        type: "separator",
      },
      {
        id: "problems",
        type: "action",
        label: "Problems",
        shortcut: "Ctrl+Shift+M",
      },
      {
        id: "output",
        type: "action",
        label: "Output",
        shortcut: "Ctrl+Shift+U",
      },
      {
        id: "terminal",
        type: "action",
        label: "Terminal",
        shortcut: "Ctrl+`",
      },
    ],
  },
  {
    id: "help",
    type: "submenu",
    label: "Help",
    alignWithIcons: true,
    items: [
      {
        id: "documentation",
        type: "action",
        label: "Documentation",
      },
      {
        id: "release-notes",
        type: "action",
        label: "Release Notes",
      },
      {
        id: "separator-6",
        type: "separator",
      },
      {
        id: "about",
        type: "action",
        label: "About",
      },
    ],
  },
];
