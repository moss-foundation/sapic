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
