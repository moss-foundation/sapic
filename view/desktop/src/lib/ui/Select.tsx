import { cva } from "class-variance-authority";
import { ComponentPropsWithoutRef, createContext, ElementRef, forwardRef, useContext } from "react";
import { twMerge } from "tailwind-merge";

import { cn } from "@/utils";
import * as SelectPrimitive from "@radix-ui/react-select";

import { Icon, Icons } from "./Icon";

const SelectGroup = SelectPrimitive.Group;

const SelectContext = createContext({
  variant: "outlined",
});

const SelectScrollUpButton = forwardRef<
  ElementRef<typeof SelectPrimitive.ScrollUpButton>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.ScrollUpButton>
>(({ className, children, ...props }, forwardedRef) => (
  <SelectPrimitive.ScrollUpButton {...props} ref={forwardedRef} className={cn(className)}>
    {children || <Icon icon="ChevronUp" className="size-3" />}
  </SelectPrimitive.ScrollUpButton>
));

const SelectScrollDownButton = forwardRef<
  ElementRef<typeof SelectPrimitive.ScrollDownButton>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.ScrollDownButton>
>(({ className, children, ...props }, forwardedRef) => (
  <SelectPrimitive.ScrollDownButton {...props} ref={forwardedRef} className={cn(className)}>
    {children || <Icon icon="ChevronDown" className="size-3" />}
  </SelectPrimitive.ScrollDownButton>
));

export interface SelectTriggerProps extends ComponentPropsWithoutRef<typeof SelectPrimitive.Trigger> {
  disabled?: boolean;
}

const selectTriggerStyles = cva(
  "relative flex cursor-pointer items-center justify-center rounded-sm px-3 transition duration-150 ease-in-out focus-visible:outline-2 focus-visible:outline-offset-2",
  {
    variants: {
      disabled: {
        true: "cursor-not-allowed grayscale-70 hover:brightness-100 active:pointer-events-none active:brightness-100",
        false: "",
      },
    },
  }
);

const SelectTrigger = forwardRef<ElementRef<typeof SelectPrimitive.Trigger>, SelectTriggerProps>(
  ({ disabled = false, className, children, ...props }, forwardedRef) => {
    return (
      <SelectPrimitive.Trigger
        {...props}
        ref={forwardedRef}
        disabled={disabled}
        className={cn(selectTriggerStyles({ disabled }), className)}
      >
        {children}
      </SelectPrimitive.Trigger>
    );
  }
);

const SelectContent = forwardRef<
  ElementRef<typeof SelectPrimitive.Content>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Content>
>(({ className, align = "start", position = "popper", children, ...props }, forwardedRef) => {
  const { variant: contextVariant } = useContext(SelectContext);

  return (
    <SelectContext.Provider value={{ variant: contextVariant }}>
      <SelectPrimitive.Content
        {...props}
        align={align}
        position={position}
        ref={forwardedRef}
        className={cn(`z-50 rounded-lg border px-3 py-1.5 shadow-lg`, className)}
      >
        {children}
      </SelectPrimitive.Content>
    </SelectContext.Provider>
  );
});

const SelectItemIndicator = forwardRef<
  ElementRef<typeof SelectPrimitive.ItemIndicator>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.ItemIndicator> & { icon?: Icons }
>(({ className, icon = "Checkmark", ...props }, forwardedRef) => (
  <SelectPrimitive.ItemIndicator
    {...props}
    ref={forwardedRef}
    className={cn(`absolute left-1.5 inline-flex size-4 items-center justify-center`, className)}
  >
    <Icon icon={icon} className="size-2.5" />
  </SelectPrimitive.ItemIndicator>
));

const selectItemStyles = cva(
  `relative flex items-center gap-1.5 rounded py-1 pr-5 pl-[7px] outline-none select-none data-[disabled]:cursor-not-allowed data-[disabled]:grayscale-100 data-[highlighted]:cursor-pointer`
);

const SelectItem = forwardRef<
  ElementRef<typeof SelectPrimitive.Item>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Item>
>(({ className, children, ...props }, forwardedRef) => {
  return (
    <SelectPrimitive.Item {...props} ref={forwardedRef} className={cn(selectItemStyles(), className)}>
      {children}
    </SelectPrimitive.Item>
  );
});

const SelectLabel = forwardRef<
  ElementRef<typeof SelectPrimitive.Label>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Label>
>(({ className, ...props }, forwardedRef) => (
  <SelectPrimitive.Label {...props} ref={forwardedRef} className={twMerge(className)} />
));

const SelectSeparator = forwardRef<
  ElementRef<typeof SelectPrimitive.Separator>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Separator>
>(({ className, ...props }, forwardedRef) => {
  return <SelectPrimitive.Separator {...props} ref={forwardedRef} className={cn(className)} />;
});

const Select = {
  Root: SelectPrimitive.Root,
  Trigger: SelectTrigger,
  Content: SelectContent,
  Item: SelectItem,
  Group: SelectGroup,
  Separator: SelectSeparator,
  Portal: SelectPrimitive.Portal,
  Value: SelectPrimitive.Value,
  ItemIndicator: SelectItemIndicator,
  ScrollUpButton: SelectScrollUpButton,
  ScrollDownButton: SelectScrollDownButton,
  Label: SelectLabel,
  Viewport: SelectPrimitive.Viewport,
  ItemText: SelectPrimitive.ItemText,
};

export default Select;
