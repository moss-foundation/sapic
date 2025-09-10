import { cva } from "class-variance-authority";
import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { Icon } from "@/lib/ui";
import SelectPrimitive, { SelectTriggerProps } from "@/lib/ui/Select";
import { cn } from "@/utils";

//prettier-ignore
const selectTriggerStyles = cva(`
    flex justify-between w-30 py-1.25 px-2 

    outline-(--moss-primary)

    border border-(--moss-mossSelect-border) 

    data-[state=open]:border-(--moss-primary)

    text-(--moss-mossSelect-text)

    data-[invalid]:border-(--moss-error)
    focus:data-[invalid]:outline-(--moss-error)

    data-[valid]:border-(--moss-success)
    focus:data-[valid]:outline-(--moss-success) 

    disabled:background-(--moss-mossSelect-disabled-bg)
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

const selectContentStyles = cva(`background-(--moss-mossSelect-bg) w-56 border-(--moss-mossSelect-border)`);

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
  `data-[highlighted]:background-(--moss-mossSelect-item-bg-hover) data-[state=checked]:background-(--moss-mossSelect-item-bg-selected) leading-5`
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
    <SelectPrimitive.Separator
      {...props}
      ref={forwardedRef}
      className={cn("background-(--moss-border-color)", className)}
    />
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
