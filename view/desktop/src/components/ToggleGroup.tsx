import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
import { cn } from "@/utils";

interface ToggleGroupRootProps {
  className?: string;
}

const toggleGroupStyles =
  "flex items-center rounded bg-[var(--moss-toggleGroup-color)] border border-[var(--moss-toggleGroup-border-color)]";

const Root = forwardRef<
  ElementRef<typeof ToggleGroupPrimitive.Root>,
  ComponentPropsWithoutRef<typeof ToggleGroupPrimitive.Root> & ToggleGroupRootProps
>(({ className, ...props }, ref) => {
  return <ToggleGroupPrimitive.Root className={cn(toggleGroupStyles, className)} {...props} ref={ref} />;
});

Root.displayName = "ToggleGroup.Root";

interface ToggleGroupItemProps {
  className?: string;
}

const toggleItemStyles = cn(
  "group flex h-[24px] px-3 cursor-pointer items-center justify-center text-md",
  "text-[var(--moss-not-selected-item-color)]",
  "data-[state=on]:bg-white data-[state=on]:text-[var(--moss-primary-text)]",
  "focus-visible:outline-none",
  "disabled:cursor-default disabled:opacity-50"
);

const Item = forwardRef<
  ElementRef<typeof ToggleGroupPrimitive.Item>,
  ComponentPropsWithoutRef<typeof ToggleGroupPrimitive.Item> & ToggleGroupItemProps
>(({ className, ...props }, ref) => {
  return <ToggleGroupPrimitive.Item className={cn(toggleItemStyles, className)} {...props} ref={ref} />;
});

Item.displayName = "ToggleGroup.Item";

export { Root, Item };

// Export as a namespace
const ToggleGroup = { Root, Item };
export default ToggleGroup;
