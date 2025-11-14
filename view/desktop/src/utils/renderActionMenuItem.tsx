import { Icons } from "@/lib/ui";
import { ActionMenu } from "@/workbench/ui/components";

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

export const renderActionMenuItem = (item: MenuItemProps, callback?: (id: string) => void) => {
  switch (item.type) {
    case "action":
      return (
        <ActionMenu.Item
          key={item.id}
          className="flex items-center justify-between"
          shortcut={item.shortcut}
          icon={item.icon || undefined}
          alignWithIcons={item.alignWithIcons || false}
          onSelect={() => callback?.(item.id)}
        >
          {item.label}
        </ActionMenu.Item>
      );

    case "submenu":
      return (
        <ActionMenu.Sub key={item.id}>
          <ActionMenu.SubTrigger
            icon={item.icon || undefined}
            onClick={() => callback?.(item.id)}
            alignWithIcons={item.alignWithIcons || false}
          >
            {item.label}
          </ActionMenu.SubTrigger>
          <ActionMenu.SubContent>
            {item.items?.map((item) => renderActionMenuItem(item, callback))}
          </ActionMenu.SubContent>
        </ActionMenu.Sub>
      );
    case "separator":
      return <ActionMenu.Separator key={item.id} />;

    case "header":
      return <ActionMenu.Label key={item.id}>{item.label}</ActionMenu.Label>;

    case "section":
      return <ActionMenu.SectionLabel key={item.id}>{item.sectionTitle}</ActionMenu.SectionLabel>;

    case "checkable":
      return (
        <ActionMenu.CheckboxItem
          key={item.id}
          className="flex items-center justify-between"
          shortcut={item.shortcut}
          checked={item.checked}
        >
          <span>{item.label}</span>
        </ActionMenu.CheckboxItem>
      );

    case "footer":
      return <ActionMenu.Footer key={item.id}>{item.footerText}</ActionMenu.Footer>;

    case "accordion":
      return (
        <ActionMenu.Accordion key={item.id}>
          <ActionMenu.AccordionTrigger total={item.items?.length}>{item.label}</ActionMenu.AccordionTrigger>
          <ActionMenu.AccordionContent>
            {item.items?.map((item) => renderActionMenuItem(item, callback))}
          </ActionMenu.AccordionContent>
        </ActionMenu.Accordion>
      );

    default:
      return null;
  }
};
