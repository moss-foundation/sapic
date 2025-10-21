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
  placeholder?: string;
  childrenLeftSide?: React.ReactNode;
  childrenRightSide?: React.ReactNode;
}

//prettier-ignore
const selectTriggerStyles = cva(`
    flex justify-between w-30 py-1.25 px-2  
    relative cursor-pointer items-center rounded-sm 
    transition duration-150 ease-in-out 

    background-(--moss-controls-background)
    border border-(--moss-controls-border) 
    text-(--moss-controls-foreground)

    data-[state=open]:border-(--moss-accent)

    data-[invalid]:border-(--moss-error)
    focus:data-[invalid]:outline-(--moss-error)

    data-[valid]:border-(--moss-success)
    focus:data-[valid]:outline-(--moss-success) 

    disabled:background-(--moss-background-disabled)
    disabled:text-(--moss-foreground-disabled)
    disabled:cursor-not-allowed

    focus-visible:ring-2 
    focus-visible:ring-(--moss-accent) 
    focus-visible:ring-offset-2
  `,
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
  (
    { disabled = false, className, placeholder, childrenLeftSide, childrenRightSide, children, ...props },
    forwardedRef
  ) => {
    return (
      <SelectPrimitive.Trigger
        {...props}
        ref={forwardedRef}
        disabled={disabled}
        className={selectTriggerStyles({ disabled, className })}
      >
        <div className="flex grow items-center gap-2 overflow-hidden">
          {childrenLeftSide}

          {children}

          <span className="min-w-0 flex-1 truncate text-left">
            <SelectPrimitive.Value placeholder={placeholder} />
          </span>

          {childrenRightSide}

          <Icon icon="ChevronDown" />
        </div>
      </SelectPrimitive.Trigger>
    );
  }
);

const SelectContent = forwardRef<
  ElementRef<typeof SelectPrimitive.Content>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Content>
>(({ className, align = "start", position = "popper", sideOffset = 6, children, ...props }, forwardedRef) => {
  const { variant: contextVariant } = useContext(SelectContext);

  return (
    <SelectContext.Provider value={{ variant: contextVariant }}>
      <SelectPrimitive.Content
        {...props}
        align={align}
        position={position}
        sideOffset={sideOffset}
        ref={forwardedRef}
        className={cn(
          `background-(--moss-controls-background) z-50 w-56 rounded-lg border border-(--moss-controls-border) px-1.5 py-1.5 shadow-lg`,
          className
        )}
      >
        <SelectPrimitive.Viewport>{children}</SelectPrimitive.Viewport>
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
  `data-[highlighted]:background-(--moss-controls-background-hover) data-[state=checked]:background-(--moss-accent-secondary) relative flex min-w-0 items-center gap-1.5 rounded py-0.5 pr-2 pl-[7px] leading-5 outline-none select-none data-[disabled]:cursor-not-allowed data-[disabled]:grayscale-100 data-[highlighted]:cursor-pointer`
);

const SelectItem = forwardRef<
  ElementRef<typeof SelectPrimitive.Item>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Item>
>(({ className, children, ...props }, forwardedRef) => {
  return (
    <SelectPrimitive.Item ref={forwardedRef} className={cn(selectItemStyles(), className)} {...props}>
      <span className="min-w-0 truncate">
        <SelectPrimitive.ItemText>{children}</SelectPrimitive.ItemText>
      </span>
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
  return (
    <SelectPrimitive.Separator
      {...props}
      ref={forwardedRef}
      className={cn("background-(--moss-border) my-0.5 h-px w-full", className)}
    />
  );
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
