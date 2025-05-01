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
