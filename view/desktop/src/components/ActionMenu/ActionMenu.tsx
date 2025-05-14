import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

const Root = Menu.Root;
const Trigger = Menu.Trigger;
const Portal = Menu.Portal;

const Content = forwardRef<HTMLDivElement, Menu.ContentProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Content
      ref={ref}
      className={cn("background-(--moss-primary-background) w-60 border border-(--moss-border-color)", className)}
      {...props}
    >
      {children}
    </Menu.Content>
  );
});

const Item = forwardRef<HTMLDivElement, Menu.ItemProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Item
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      shortcutClassName="text-(--moss-shortcut-text)"
      {...props}
    >
      {children}
    </Menu.Item>
  );
});

export { Content, Item, Portal, Root, Trigger };
