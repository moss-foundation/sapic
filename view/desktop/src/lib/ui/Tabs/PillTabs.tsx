import { ReactNode } from "react";

import { cn } from "@/utils";

import { TabsPrimitive } from ".";

interface PillTabsProps {
  value?: string | null;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: PillTabsProps) => {
  return (
    <TabsPrimitive.Tabs
      value={value ?? undefined}
      onValueChange={onValueChange}
      className={cn("flex min-h-fit grow flex-col", className)}
    >
      {children}
    </TabsPrimitive.Tabs>
  );
};

interface PillTabsListProps {
  children: ReactNode;
  className?: string;
}

const List = ({ children, className }: PillTabsListProps) => {
  return (
    <TabsPrimitive.TabsList className={cn("w-full p-0.5", className)} data-tabs-list-container>
      {children}
    </TabsPrimitive.TabsList>
  );
};

interface PillTabProps {
  value: string;
  className?: string;
  trailingContent?: ReactNode;
  leadingContent?: ReactNode;
  label: string;
  disabled?: boolean;
}

const Trigger = ({ value, className, trailingContent, leadingContent, label, disabled }: PillTabProps) => {
  return (
    <TabsPrimitive.TabsTrigger
      value={value}
      className={cn(
        "border-none! p-0 text-base leading-4",
        "cursor-pointer rounded-full",

        "outline-(--moss-accent)",
        "focus-visible:outline-3 focus-visible:outline-offset-1",

        "ring-(--moss-border) ring",
        "hover:not-data-[state=active]:hover:ring-(--moss-secondary-background-hover)",

        "data-[state=active]:ring-2 data-[state=active]:ring-offset-0",
        "data-[state=active]:ring-(--moss-accent)",

        "disabled:data-[state=active]:ring-(--moss-gray-11)",

        className
      )}
      disabled={disabled}
    >
      <div
        className={cn("flex cursor-pointer items-center gap-[5px] py-2", {
          "px-2": leadingContent && trailingContent,
          "pl-2 pr-3": leadingContent && !trailingContent,
          "pl-3 pr-2": !leadingContent && trailingContent,
        })}
      >
        {leadingContent}
        <span>{label}</span>
        {trailingContent}
      </div>
    </TabsPrimitive.TabsTrigger>
  );
};

interface PillTabContentProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Content = ({ value, children, className }: PillTabContentProps) => {
  return (
    <TabsPrimitive.TabsContent value={value} className={cn(className)}>
      {children}
    </TabsPrimitive.TabsContent>
  );
};

const PillTabs = {
  Root,
  List,
  Trigger,
  Content,
};

export default PillTabs;
