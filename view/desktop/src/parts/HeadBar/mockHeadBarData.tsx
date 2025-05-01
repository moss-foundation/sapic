import { type Icons } from "@/components/Icon";
import { type MenuItemProps } from "@/components/ActionMenu/ActionMenu";

// Mock user menu items
export const userMenuItems: MenuItemProps[] = [
  {
    id: "user-profile",
    type: "action",
    label: "Profile",
    icon: "HeadBarUserAvatar" as Icons,
  },
  {
    id: "user-settings",
    type: "action",
    label: "User Settings",
    icon: "HeadBarSettings" as Icons,
  },
  {
    id: "separator",
    type: "separator",
  },
  {
    id: "status",
    type: "submenu",
    label: "Status",
    icon: "TestHeadBarLogs" as Icons,
    items: [
      {
        id: "status-online",
        type: "action",
        label: "Online",
        icon: "CheckboxIndicator" as Icons,
      },
      {
        id: "status-away",
        type: "action",
        label: "Away",
      },
      {
        id: "status-do-not-disturb",
        type: "action",
        label: "Do Not Disturb",
      },
    ],
  },
  {
    id: "separator-2",
    type: "separator",
  },
  {
    id: "logout",
    type: "action",
    label: "Log Out",
    variant: "danger",
  },
];

// Mock git branch menu items
export const gitBranchMenuItems: MenuItemProps[] = [
  {
    id: "current-branch",
    type: "header",
    label: "Current Branch",
  },
  {
    id: "main",
    type: "action",
    label: "main",
    icon: "CheckboxIndicator" as Icons,
  },
  {
    id: "separator-1",
    type: "separator",
  },
  {
    id: "local-branches",
    type: "section",
    sectionTitle: "Local Branches",
  },
  {
    id: "develop",
    type: "action",
    label: "develop",
  },
  {
    id: "feature/user-auth",
    type: "action",
    label: "feature/user-auth",
  },
  {
    id: "bugfix/login-issue",
    type: "action",
    label: "bugfix/login-issue",
  },
  {
    id: "separator-2",
    type: "separator",
  },
  {
    id: "remote-branches",
    type: "section",
    sectionTitle: "Remote Branches",
  },
  {
    id: "origin/main",
    type: "action",
    label: "origin/main",
  },
  {
    id: "origin/develop",
    type: "action",
    label: "origin/develop",
  },
  {
    id: "separator-3",
    type: "separator",
  },
  {
    id: "git-actions",
    type: "section",
    sectionTitle: "Git Actions",
  },
  {
    id: "create-branch",
    type: "action",
    label: "Create New Branch...",
    icon: "AddCircle" as Icons,
  },
  {
    id: "pull",
    type: "action",
    label: "Pull",
  },
  {
    id: "push",
    type: "action",
    label: "Push",
  },
];

// Mock Windows menu items
export const windowsMenuItems: MenuItemProps[] = [
  {
    id: "file",
    type: "submenu",
    label: "File",
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
