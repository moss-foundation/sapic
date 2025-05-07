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
import * as DropdownMenuPrimitive from "@/lib/ui/Menu/DropdownMenu/DropdownMenu";
import { cn } from "@/utils";

const Root: typeof DropdownMenuPrimitive.Root = DropdownMenuPrimitive.Root;
const Trigger: typeof DropdownMenuPrimitive.Trigger = DropdownMenuPrimitive.Trigger;
const Portal: typeof DropdownMenuPrimitive.Portal = DropdownMenuPrimitive.Portal;

const Content = forwardRef<HTMLDivElement, ContentProps>(({ children, className, ...props }, ref) => {
  return (
    <DropdownMenuPrimitive.Content
      ref={ref}
      className={cn("background-(--moss-primary-background) min-w-48 border border-(--moss-border-color)", className)}
      {...props}
    >
      {children}
    </DropdownMenuPrimitive.Content>
  );
});

const Item = forwardRef<HTMLDivElement, ItemProps>(({ children, className, ...props }, ref) => {
  return (
    <DropdownMenuPrimitive.Item
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    >
      {children}
    </DropdownMenuPrimitive.Item>
  );
});

const Separator = () => {
  return <DropdownMenuPrimitive.Separator className="background-(--moss-border-color)" />;
};

const CheckboxItem = forwardRef<HTMLDivElement, CheckboxItemProps>(({ children, className, ...props }, ref) => {
  return (
    <DropdownMenuPrimitive.CheckboxItem
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    >
      {props.checked ? <Icon icon="GreenCheckmark" /> : <Icon icon="GreenCheckmark" className="opacity-0" />}

      <div className="flex w-full items-center gap-2.5">
        <span>{props.label}</span>
      </div>
    </DropdownMenuPrimitive.CheckboxItem>
  );
});

const RadioGroup = DropdownMenuPrimitive.RadioGroup;
const RadioItem = forwardRef<HTMLDivElement, RadioItemProps>(({ children, className, ...props }, ref) => {
  return (
    <DropdownMenuPrimitive.RadioItem
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    />
  );
});

const Sub: typeof DropdownMenuPrimitive.Sub = DropdownMenuPrimitive.Sub;

const SubTrigger = forwardRef<HTMLDivElement, SubTriggerProps>(({ children, className, ...props }, ref) => {
  return (
    <DropdownMenuPrimitive.SubTrigger
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    >
      {children}
    </DropdownMenuPrimitive.SubTrigger>
  );
});

const SubContent = forwardRef<HTMLDivElement, SubContentProps>(({ children, className, ...props }, ref) => {
  return (
    <DropdownMenuPrimitive.SubContent
      ref={ref}
      className={cn("background-(--moss-primary-background) border border-(--moss-border-color)", className)}
      {...props}
    >
      {children}
    </DropdownMenuPrimitive.SubContent>
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
