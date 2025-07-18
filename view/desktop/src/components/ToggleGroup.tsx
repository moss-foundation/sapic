import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";

interface ToggleGroupRootProps {
  className?: string;
}

const toggleGroupStyles = "flex items-center rounded background-(--moss-headBar-primary-background)";

const Root = forwardRef<
  ElementRef<typeof ToggleGroupPrimitive.Root>,
  ComponentPropsWithoutRef<typeof ToggleGroupPrimitive.Root> & ToggleGroupRootProps
>(({ className, ...props }, ref) => {
  return <ToggleGroupPrimitive.Root className={cn(toggleGroupStyles, className)} {...props} ref={ref} />;
});

Root.displayName = "ToggleGroup.Root";

interface ToggleGroupItemProps {
  className?: string;
  compact?: boolean;
  children?: React.ReactNode;
}

const toggleItemStyles = cn(
  "group text-md flex h-[24px] cursor-pointer items-center justify-center px-3",
  "text-[var(--moss-not-selected-item-color)]",
  "data-[state=on]:text-[var(--moss-primary-text)]",
  "focus-visible:outline-none",
  "disabled:cursor-default disabled:opacity-50"
);

const Item = forwardRef<
  ElementRef<typeof ToggleGroupPrimitive.Item>,
  ComponentPropsWithoutRef<typeof ToggleGroupPrimitive.Item> & ToggleGroupItemProps
>(({ className, compact, children, ...props }, ref) => {
  const displayText = compact && typeof children === "string" ? children.charAt(0) : children;

  return (
    <ToggleGroupPrimitive.Item className={cn(toggleItemStyles, className)} {...props} ref={ref}>
      {displayText}
    </ToggleGroupPrimitive.Item>
  );
});

Item.displayName = "ToggleGroup.Item";

export { Item, Root };

const ToggleGroup = { Root, Item };
export default ToggleGroup;
