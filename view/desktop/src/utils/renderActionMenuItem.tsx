import { ActionMenu } from "@/components";
import { MenuItemProps } from "@/components/ActionMenu/ActionMenu";

export const renderActionMenuItem = (item: MenuItemProps, callback: (id: string) => void) => {
  switch (item.type) {
    case "action":
      return (
        <ActionMenu.Item
          key={item.id}
          className="flex items-center justify-between"
          shortcut={item.shortcut}
          icon={item.icon || undefined}
          leftIconPadding={item.alignWithIcons}
          onSelect={() => callback?.(item.id)}
        >
          <span>{item.label}</span>
        </ActionMenu.Item>
      );
    case "separator":
      return <ActionMenu.Separator key={item.id} />;
    case "submenu":
      return (
        <ActionMenu.Sub key={item.id}>
          <ActionMenu.SubTrigger icon={item.icon || undefined} onClick={() => callback?.(item.id)}>
            {item.label}
          </ActionMenu.SubTrigger>
          <ActionMenu.SubContent>
            {item.items?.map((item) => renderActionMenuItem(item, callback))}
          </ActionMenu.SubContent>
        </ActionMenu.Sub>
      );
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
    case "accordion":
      return (
        <ActionMenu.Accordion key={item.id}>
          <ActionMenu.AccordionTrigger>{item.label}</ActionMenu.AccordionTrigger>
          <ActionMenu.AccordionContent>
            {item.items?.map((item) => renderActionMenuItem(item, callback))}
          </ActionMenu.AccordionContent>
        </ActionMenu.Accordion>
      );
    default:
      return null;
  }
};
