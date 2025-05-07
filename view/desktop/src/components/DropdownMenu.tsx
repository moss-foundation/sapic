import { forwardRef } from "react";

import { Icon } from "@/lib/ui";
import { CheckboxItemProps, ContentProps, ItemProps, RadioItemProps, SubContentProps } from "@/lib/ui/Menu";
import * as DropdownMenuPrimitive from "@/lib/ui/Menu/DropdownMenu/DropdownMenu";
import { cn } from "@/utils";

const Root = DropdownMenuPrimitive.Root;
const Trigger = DropdownMenuPrimitive.Trigger;
const Portal = DropdownMenuPrimitive.Portal;

const Content = forwardRef<HTMLDivElement, ContentProps>(({ children, className, ...props }, ref) => {
  return (
    <DropdownMenuPrimitive.Content
      ref={ref}
      className={cn("background-(--moss-primary-background) border border-(--moss-border-color)", className)}
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
      className={cn("hover:background-(--moss-primary-background-hover)", className)}
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
      className={cn("hover:background-(--moss-primary-background-hover)", className)}
      {...props}
    >
      {props.checked ? (
        <Icon icon="RadioIndicator" className="h-4 w-4" />
      ) : (
        <Icon icon="RadioIndicator" className="h-4 w-4 opacity-0" />
      )}

      <div className="flex w-full items-center gap-2.5">
        <span>{props.label}</span>
      </div>
    </DropdownMenuPrimitive.RadioItem>
  );
});

const Sub = DropdownMenuPrimitive.Sub;

const SubTrigger = DropdownMenuPrimitive.SubTrigger;

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

const DropdownMenu = {
  Root,
  Trigger,
  Portal,
  Content,
  Item,
  Separator,
  Sub,
  SubTrigger,
  SubContent,
  CheckboxItem,
  RadioGroup,
  RadioItem,
};

export default DropdownMenu as {
  Root: typeof DropdownMenuPrimitive.Root;
  Trigger: typeof DropdownMenuPrimitive.Trigger;
  Portal: typeof DropdownMenuPrimitive.Portal;
  Content: typeof Content;
  Item: typeof Item;
  Separator: typeof Separator;
  Sub;
  SubTrigger: typeof SubTrigger;
  SubContent: typeof SubContent;
  CheckboxItem: typeof CheckboxItem;
  RadioGroup: typeof RadioGroup;
  RadioItem: typeof RadioItem;
};
