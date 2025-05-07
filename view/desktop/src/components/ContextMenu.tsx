import { forwardRef } from "react";

import { Icon } from "@/lib/ui";
import {
  CheckboxItemProps,
  ContentProps,
  ItemProps,
  RadioItemProps,
  SubContentProps,
  SubTriggerProps,
} from "@/lib/ui/Menu";
import { cn } from "@/utils";

import * as ContextMenuPrimitive from "../lib/ui/Menu/ContextMenu/ContextMenu";

const Root = ContextMenuPrimitive.Root;
const Trigger = ContextMenuPrimitive.Trigger;
const Portal = ContextMenuPrimitive.Portal;

const Content = forwardRef<HTMLDivElement, ContentProps>(({ children, className, ...props }, ref) => {
  return (
    <ContextMenuPrimitive.Content
      ref={ref}
      className={cn("background-(--moss-primary-background) border border-(--moss-border-color)", className)}
      {...props}
    >
      {children}
    </ContextMenuPrimitive.Content>
  );
});

const Item = forwardRef<HTMLDivElement, ItemProps>(({ children, className, ...props }, ref) => {
  return (
    <ContextMenuPrimitive.Item
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    >
      {children}
    </ContextMenuPrimitive.Item>
  );
});

const Separator = () => {
  return <ContextMenuPrimitive.Separator className="background-(--moss-border-color)" />;
};

const CheckboxItem = forwardRef<HTMLDivElement, CheckboxItemProps>(({ children, className, ...props }, ref) => {
  return (
    <ContextMenuPrimitive.CheckboxItem
      ref={ref}
      className={cn("hover:background-(--moss-primary-background-hover)", className)}
      {...props}
    >
      {props.checked ? <Icon icon="GreenCheckmark" /> : <Icon icon="GreenCheckmark" className="opacity-0" />}

      <div className="flex w-full items-center gap-2.5">
        <span>{props.label}</span>

        {props.shortcut && <div className="ml-auto opacity-30">{props.shortcut.join("")}</div>}
      </div>
    </ContextMenuPrimitive.CheckboxItem>
  );
});

const RadioGroup = ContextMenuPrimitive.RadioGroup;
const RadioItem = forwardRef<HTMLDivElement, RadioItemProps>(({ children, className, ...props }, ref) => {
  return (
    <ContextMenuPrimitive.RadioItem
      ref={ref}
      className={cn("hover:background-(--moss-primary-background-hover)", className)}
      {...props}
    />
  );
});

const Sub = ContextMenuPrimitive.Sub;

const SubTrigger = forwardRef<HTMLDivElement, SubTriggerProps>(({ children, className, ...props }, ref) => {
  return (
    <ContextMenuPrimitive.SubTrigger
      ref={ref}
      className={cn("hover:background-(--moss-primary-background-hover)", className)}
      {...props}
    >
      {children}
    </ContextMenuPrimitive.SubTrigger>
  );
});

const SubContent = forwardRef<HTMLDivElement, SubContentProps>(({ children, className, ...props }, ref) => {
  return (
    <ContextMenuPrimitive.SubContent
      ref={ref}
      className={cn("background-(--moss-primary-background) border border-(--moss-border-color)", className)}
      {...props}
    >
      {children}
    </ContextMenuPrimitive.SubContent>
  );
});

export {
  CheckboxItem,
  Content,
  Item,
  Portal,
  RadioGroup,
  RadioItem,
  Root,
  Separator,
  Sub,
  SubContent,
  SubTrigger,
  Trigger,
};
