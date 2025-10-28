import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as RadixTabsPrimitive from "@radix-ui/react-tabs";

const Tabs = RadixTabsPrimitive.Root;

const TabsList = forwardRef<
  ElementRef<typeof RadixTabsPrimitive.List>,
  ComponentPropsWithoutRef<typeof RadixTabsPrimitive.List>
>(({ className, ...props }, ref) => (
  <RadixTabsPrimitive.List ref={ref} className={cn("flex items-center", className)} {...props} />
));

const TabsTrigger = forwardRef<
  ElementRef<typeof RadixTabsPrimitive.Trigger>,
  ComponentPropsWithoutRef<typeof RadixTabsPrimitive.Trigger>
>(({ className, ...props }, ref) => (
  <RadixTabsPrimitive.Trigger
    ref={ref}
    className={cn(
      "flex shrink-0 items-center gap-1.5 px-3 py-1.5 text-sm transition-colors",
      "border-0 border-transparent",
      "text-(--moss-secondary-foreground) hover:text-(--moss-primary-foreground)",
      "data-[state=active]:text-(--moss-primary-foreground)",
      "disabled:pointer-events-none disabled:opacity-50",
      className
    )}
    {...props}
  />
));

const TabsContent = forwardRef<
  ElementRef<typeof RadixTabsPrimitive.Content>,
  ComponentPropsWithoutRef<typeof RadixTabsPrimitive.Content>
>(({ className, ...props }, ref) => (
  <RadixTabsPrimitive.Content ref={ref} className={cn("outline-none", className)} {...props} />
));

const TabsPrimitive = {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
};

export default TabsPrimitive;
