import { cva } from "class-variance-authority";
import { ComponentPropsWithoutRef, createContext, ElementRef, forwardRef, useContext } from "react";
import { twMerge } from "tailwind-merge";

import { cn } from "@/utils";
import * as SelectPrimitive from "@radix-ui/react-select";

import Icon, { Icons } from "../lib/ui/Icon";

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
  size?: "xs" | "sm" | "md" | "lg" | "xl";
  variant?: "outlined" | "soft" | "mixed" | "bottomOutlined";
  disabled?: boolean;
}

const selectTriggerStyles = cva(
  "relative flex cursor-pointer items-center justify-center rounded-sm px-3 text-sm outline-blue-600 transition duration-150 ease-in-out focus-visible:outline-2 focus-visible:outline-offset-2",
  {
    variants: {
      variant: {
        outlined: `border border-(--moss-select-border-outlined) text-(--moss-select-text-outlined) data-[invalid]:border-[rgb(239,68,68)] focus:data-[invalid]:outline-[rgb(239,68,68)] data-[valid]:border-[rgb(22,163,74)] focus:data-[valid]:outline-[rgb(22,163,74)]`,
        soft: `background-(--moss-select-bg-soft) text-(--moss-select-text-soft) outline-none focus:brightness-95 data-[invalid]:bg-[rgb(254,226,226)] data-[valid]:bg-[rgb(220,252,231)] dark:focus:brightness-105 dark:data-[invalid]:bg-[rgb(153,27,27,0.25)] dark:data-[valid]:bg-[rgba(22,101,52,0.25)]`,
        mixed: `background-(--moss-select-bg-mixed) border border-(--moss-select-border-mixed) text-(--moss-select-text-mixed) shadow-sm shadow-gray-900/5 data-[invalid]:border-[rgb(220,38,38)] focus:data-[invalid]:outline-[rgb(220,38,38)] data-[valid]:border-[rgb(22,163,74)] focus:data-[valid]:outline-[rgb(22,163,74)] dark:shadow-gray-900/35`,
        bottomOutlined: `rounded-none border-b border-(--moss-select-border-bottomOutlined) px-0 text-(--moss-select-text-bottomOutlined) transition-[border] focus:border-b-2 focus:border-[rgb(37,99,235)] data-[invalid]:border-[rgb(248,113,113)] data-[valid]:border-[rgb(74,222,128)]`,
      },
      size: {
        xs: "h-6",
        sm: "h-7",
        md: "h-8",
        lg: "h-9",
        xl: "h-10",
      },
      disabled: {
        true: "cursor-not-allowed grayscale-70 hover:brightness-100 active:pointer-events-none active:brightness-100",
        false: "",
      },
    },
    defaultVariants: {
      variant: "outlined",
      size: "md",
      disabled: false,
    },
  }
);

const SelectTrigger = forwardRef<ElementRef<typeof SelectPrimitive.Trigger>, SelectTriggerProps>(
  ({ size = "sm", variant = "outlined", disabled = false, className, children, ...props }, forwardedRef) => {
    return (
      <SelectPrimitive.Trigger
        {...props}
        ref={forwardedRef}
        disabled={disabled}
        className={cn(selectTriggerStyles({ size, variant, disabled }), className)}
      >
        {children}
      </SelectPrimitive.Trigger>
    );
  }
);

const SelectContent = forwardRef<
  ElementRef<typeof SelectPrimitive.Content>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Content>
>(({ className, children, ...props }, forwardedRef) => {
  const { variant: contextVariant } = useContext(SelectContext);

  return (
    <SelectContext.Provider value={{ variant: contextVariant }}>
      <SelectPrimitive.Content
        {...props}
        ref={forwardedRef}
        className={cn(
          `z-50 rounded-lg border border-[#c9ccd6] bg-white px-3 py-1.5 shadow-lg dark:border-[rgb(45,45,50)] dark:bg-[rgb(24,24,27)]`,
          className
        )}
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
  `relative flex items-center gap-1.5 rounded py-1 pr-5 pl-[7px] text-gray-700 outline-none select-none data-[disabled]:cursor-not-allowed data-[disabled]:grayscale-100 data-[highlighted]:cursor-pointer data-[highlighted]:bg-[#D4E2FF] dark:text-gray-300 dark:data-[highlighted]:bg-[rgb(34,34,36)]`
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
