import { cva } from "class-variance-authority";
import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import SelectPrimitive, { SelectTriggerProps } from "@/lib/ui/Select";
import { cn } from "@/utils";

//prettier-ignore
const selectTriggerStyles = cva(`
    flex w-56 justify-between 

    outline-(--moss-accent)

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
 `,
  {
    variants: {
      size: {
        xs: "h-6",
        sm: "h-7",
        md: "h-8",
      },
    },
  }
);

export interface OutlinedSelectTriggerProps extends SelectTriggerProps {
  size?: "xs" | "sm" | "md";
  placeholder?: string;
}

const Trigger = forwardRef<ElementRef<typeof SelectPrimitive.Trigger>, OutlinedSelectTriggerProps>(
  ({ placeholder, disabled = false, className, size = "md", ...props }, forwardedRef) => {
    return (
      <SelectPrimitive.Trigger
        {...props}
        ref={forwardedRef}
        disabled={disabled}
        className={cn(selectTriggerStyles({ size }), className)}
      >
        <span className="truncate">
          <SelectPrimitive.Value placeholder={placeholder} />
        </span>
      </SelectPrimitive.Trigger>
    );
  }
);

const selectContentStyles = cva(`background-(--moss-controls-background) w-56 border-(--moss-controls-border)`);

const Content = forwardRef<
  ElementRef<typeof SelectPrimitive.Content>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Content>
>(({ className, children, ...props }, forwardedRef) => {
  return (
    <SelectPrimitive.Content {...props} ref={forwardedRef} className={cn(selectContentStyles(), className)}>
      <SelectPrimitive.Viewport>{children}</SelectPrimitive.Viewport>
    </SelectPrimitive.Content>
  );
});

const selectItemStyles = cva(
  `data-[highlighted]:background-(--moss-controls-background-hover) data-[state=checked]:background-(--moss-accent-secondary) leading-5`
);

const Item = forwardRef<ElementRef<typeof SelectPrimitive.Item>, ComponentPropsWithoutRef<typeof SelectPrimitive.Item>>(
  ({ className, children, ...props }, forwardedRef) => {
    return (
      <SelectPrimitive.Item {...props} ref={forwardedRef} className={cn(selectItemStyles(), className)}>
        {children}
      </SelectPrimitive.Item>
    );
  }
);

const Separator = forwardRef<
  ElementRef<typeof SelectPrimitive.Separator>,
  ComponentPropsWithoutRef<typeof SelectPrimitive.Separator>
>(({ className, ...props }, forwardedRef) => {
  return (
    <SelectPrimitive.Separator {...props} ref={forwardedRef} className={cn("background-(--moss-border)", className)} />
  );
});

const SelectOutlined = {
  Root: SelectPrimitive.Root,
  Trigger,
  Content,
  Item,
  Separator,
};

export default SelectOutlined;
