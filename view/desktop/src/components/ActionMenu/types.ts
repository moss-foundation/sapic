import { Icons } from "@/lib/ui";

export type MenuItemType =
  | "action"
  | "submenu"
  | "separator"
  | "header"
  | "section"
  | "checkable"
  | "footer"
  | "radio"
  | "accordion";

export interface MenuItemProps {
  id: string;
  type: MenuItemType;
  label?: string;
  icon?: Icons | null;
  iconColor?: string;
  shortcut?: string;
  items?: MenuItemProps[];
  disabled?: boolean;
  checked?: boolean;
  variant?: "danger" | "success" | "warning" | "info" | "default";
  sectionTitle?: string;
  footerText?: string;
  value?: string;
  alignWithIcons?: boolean;
}
