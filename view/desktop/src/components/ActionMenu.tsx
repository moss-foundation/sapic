import { forwardRef } from "react";

import { Icon, Menu } from "@/lib/ui";
import {
  CheckboxItemProps,
  ContentProps,
  ItemProps,
  RadioItemProps,
  SubContentProps,
  SubTriggerProps,
} from "@/lib/ui/Menu/Menu";
import { cn } from "@/utils";

const Root = Menu.Root;
const Trigger = Menu.Trigger;
const Portal = Menu.Portal;

const Content = forwardRef<HTMLDivElement, ContentProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Content
      ref={ref}
      className={cn("background-(--moss-primary-background) min-w-48 border border-(--moss-border-color)", className)}
      {...props}
    >
      {children}
    </Menu.Content>
  );
});

const Item = forwardRef<HTMLDivElement, ItemProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Item ref={ref} className={cn("hover:background-(--moss-secondary-background-hover)", className)} {...props}>
      {children}
    </Menu.Item>
  );
});

const Separator = () => {
  return <Menu.Separator className="background-(--moss-border-color)" />;
};

const CheckboxItem = forwardRef<HTMLDivElement, CheckboxItemProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.CheckboxItem
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    >
      {props.checked ? <Icon icon="GreenCheckmark" /> : <Icon icon="GreenCheckmark" className="opacity-0" />}

      <div className="flex w-full items-center gap-2.5">
        <span>{children}</span>

        {props.shortcut && <div className="ml-auto opacity-30">{props.shortcut.join("")}</div>}
      </div>
    </Menu.CheckboxItem>
  );
});

const RadioGroup = Menu.RadioGroup;
const RadioItem = forwardRef<HTMLDivElement, RadioItemProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.RadioItem
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    />
  );
});

const Sub = Menu.Sub;

const SubTrigger = forwardRef<HTMLDivElement, SubTriggerProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.SubTrigger
      ref={ref}
      className={cn("hover:background-(--moss-secondary-background-hover)", className)}
      {...props}
    >
      {children}
    </Menu.SubTrigger>
  );
});

const SubContent = forwardRef<HTMLDivElement, SubContentProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.SubContent
      ref={ref}
      className={cn("background-(--moss-primary-background) border border-(--moss-border-color)", className)}
      {...props}
    >
      {children}
    </Menu.SubContent>
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
