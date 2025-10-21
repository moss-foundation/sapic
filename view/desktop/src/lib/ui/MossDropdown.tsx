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
        "hover:background-(--moss-controls-background-hover) background-(--moss-controls-background) cursor-pointer rounded-md border border-(--moss-controls-border) p-1.25",
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
    <Menu.Item ref={ref} className={cn("hover:background-(--moss-controls-background-hover)", className)} {...props}>
      {children}
    </Menu.Item>
  );
});

const Content = forwardRef<HTMLDivElement, Menu.ContentProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Content
      ref={ref}
      className={cn(
        "background-(--moss-controls-background) rounded-md border border-(--moss-controls-border) p-1.25",
        className
      )}
      {...props}
    >
      {children}
    </Menu.Content>
  );
});

export { Content, Item, Portal, Root, Trigger };
