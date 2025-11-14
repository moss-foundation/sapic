import { MenuItemProps } from "@/workbench/utils/renderActionMenuItem";

// User menu items function that returns appropriate items based on whether a user is selected
export const userMenuItems: MenuItemProps[] = [
  {
    id: "user-profile",
    type: "action",
    label: "Profile",
    icon: "UserAvatar",
  },
  {
    id: "user-settings",
    type: "action",
    label: "User Settings",
    icon: "Settings",
  },
  {
    id: "separator",
    type: "separator",
  },
  {
    id: "status",
    type: "submenu",
    label: "Status",
    icon: "Placeholder",
    items: [
      {
        id: "status-online",
        type: "action",
        label: "Online",
        icon: "Checkmark",
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

// User menu items when no user is selected
export const noUserMenuItems: MenuItemProps[] = [
  {
    id: "sign-in",
    type: "action",
    label: "Sign In",
    icon: "UserAvatar",
  },
  {
    id: "create-account",
    type: "action",
    label: "Create Account",
    icon: "AddCircle",
  },
];

// Function to get user menu items based on user selection state
export const getUserMenuItems = (selectedUser: string | null): MenuItemProps[] => {
  return selectedUser ? userMenuItems : noUserMenuItems;
};

// Mock git branch menu items when a branch is selected
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
    icon: "Checkmark",
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
    icon: "AddCircle",
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

// Git branch items when no branch is selected
export const noBranchMenuItems: MenuItemProps[] = [
  {
    id: "select-branch",
    type: "action",
    label: "Select Branch",
    icon: "VCS",
  },
  {
    id: "create-branch",
    type: "action",
    label: "Create New Branch...",
    icon: "AddCircle",
  },
  {
    id: "init-repo",
    type: "action",
    label: "Initialize Repository",
    icon: "Placeholder",
  },
];

// Function to get git branch menu items based on branch selection state
export const getGitBranchMenuItems = (selectedBranch: string | null): MenuItemProps[] => {
  return selectedBranch ? gitBranchMenuItems : noBranchMenuItems;
};

// Mock Windows menu items
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
