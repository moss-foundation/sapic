import { cva } from "class-variance-authority";
import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { Icon } from "@/lib/ui";
import SelectPrimitive, { SelectTriggerProps } from "@/lib/ui/Select";
import { cn } from "@/utils";

//prettier-ignore
const selectTriggerStyles = cva(`
    flex justify-between w-30 py-1.25 px-2 

    outline-(--moss-primary)

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
 `,
);

export interface MossSelectTriggerProps extends SelectTriggerProps {
  placeholder?: string;
  childrenLeftSide?: React.ReactNode;
  childrenRightSide?: React.ReactNode;
}

const Trigger = forwardRef<ElementRef<typeof SelectPrimitive.Trigger>, MossSelectTriggerProps>(
  ({ placeholder, disabled = false, className, childrenLeftSide, childrenRightSide, ...props }, forwardedRef) => {
    return (
      <SelectPrimitive.Trigger
        {...props}
        ref={forwardedRef}
        disabled={disabled}
        className={cn(selectTriggerStyles(), className)}
      >
        <div className="flex grow items-center gap-2 overflow-hidden">
          {childrenLeftSide}

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

const SelectMoss = {
  Root: SelectPrimitive.Root,
  Trigger,
  Content,
  Item,
  Separator,
};

export default SelectMoss;
