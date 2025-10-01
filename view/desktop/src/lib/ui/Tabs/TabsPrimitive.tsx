import { ComponentPropsWithoutRef, ElementRef, forwardRef } from "react";

import { cn } from "@/utils";
import * as RadixTabsPrimitive from "@radix-ui/react-tabs";

import Scrollbar from "../Scrollbar";

const Tabs = RadixTabsPrimitive.Root;

const TabsList = forwardRef<
  ElementRef<typeof RadixTabsPrimitive.List>,
  ComponentPropsWithoutRef<typeof RadixTabsPrimitive.List>
>(({ className, ...props }, ref) => (
  <Scrollbar
    className={cn("h-auto w-full min-w-0 items-center", { "pr-2": toolbar })}
    classNames={{
      contentWrapper: "mr-2",
    }}
    data-tabs-list-container
  >
    <RadixTabsPrimitive.List ref={ref} className={cn("flex items-center", className)} {...props} />
  </Scrollbar>
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
      "text-(--moss-secondary-text) hover:text-(--moss-primary-text)",
      "data-[state=active]:text-(--moss-primary-text)",
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
