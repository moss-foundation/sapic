import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
import { cn } from "@/utils";

interface ToggleGroupRootProps {
  className?: string;
}

const toggleGroupStyles = "flex items-center rounded bg-[var(--moss-secondary-background)] overflow-hidden";

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

const toggleItemStyles =
  "group flex h-[34px] px-4 cursor-pointer items-center justify-center text-[var(--moss-primary-text)] " +
  "data-[state=on]:bg-[var(--moss-primary-background)] data-[state=on]:font-medium " +
  "hover:bg-[var(--moss-icon-primary-background-hover)] " +
  "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--moss-primary)] focus-visible:ring-offset-2 " +
  "disabled:cursor-default disabled:opacity-50";

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
