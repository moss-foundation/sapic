import { forwardRef } from "react";

import { cn } from "@/utils";

import * as Menu from "./Menu";

const Root = Menu.Root;
const Portal = Menu.Portal;

const Trigger = forwardRef<HTMLDivElement, Menu.ActionMenuTriggerProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Trigger
      ref={ref}
      className={cn(
        "hover:background-(--moss-mossDropdown-hover-bg) background-(--moss-mossDropdown-bg) cursor-pointer rounded-md border border-(--moss-mossDropdown-border) p-1.25",
        className
      )}
      {...props}
    >
      {children}
    </Menu.Trigger>
  );
});

const Item = forwardRef<HTMLDivElement, Menu.ItemProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Item ref={ref} className={cn("hover:background-(--moss-secondary-background-hover)", className)} {...props}>
      {children}
    </Menu.Item>
  );
});

const Content = forwardRef<HTMLDivElement, Menu.ContentProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Content
      ref={ref}
      className={cn(
        "background-(--moss-mossDropdown-bg) rounded-md border border-(--moss-mossDropdown-border) p-1.25",
        className
      )}
      {...props}
    >
      {children}
    </Menu.Content>
  );
});

export { Content, Item, Portal, Root, Trigger };
