import { ActionMenuRadix } from "@/components";
import { MenuItemProps } from "@/components/ActionMenu/ActionMenu";

export const renderActionMenuItem = (item: MenuItemProps, callback: (id: string) => void) => {
  switch (item.type) {
    case "action":
      return (
        <ActionMenuRadix.Item
          key={item.id}
          className="flex items-center justify-between"
          shortcut={item.shortcut}
          icon={item.icon || undefined}
          leftIconPadding={item.alignWithIcons}
          onSelect={() => callback?.(item.id)}
        >
          <span>{item.label}</span>
        </ActionMenuRadix.Item>
      );
    case "separator":
      return <ActionMenuRadix.Separator key={item.id} />;
    case "submenu":
      return (
        <ActionMenuRadix.Sub key={item.id}>
          <ActionMenuRadix.SubTrigger icon={item.icon || undefined} onClick={() => callback?.(item.id)}>
            {item.label}
          </ActionMenuRadix.SubTrigger>
          <ActionMenuRadix.SubContent>
            {item.items?.map((item) => renderActionMenuItem(item, callback))}
          </ActionMenuRadix.SubContent>
        </ActionMenuRadix.Sub>
      );
    case "checkable":
      return (
        <ActionMenuRadix.CheckboxItem
          key={item.id}
          className="flex items-center justify-between"
          shortcut={item.shortcut}
          checked={item.checked}
        >
          <span>{item.label}</span>
        </ActionMenuRadix.CheckboxItem>
      );
    case "accordion":
      return (
        <ActionMenuRadix.Accordion key={item.id}>
          <ActionMenuRadix.AccordionTrigger>{item.label}</ActionMenuRadix.AccordionTrigger>
          <ActionMenuRadix.AccordionContent>
            {item.items?.map((item) => renderActionMenuItem(item, callback))}
          </ActionMenuRadix.AccordionContent>
        </ActionMenuRadix.Accordion>
      );
    default:
      return null;
  }
};
