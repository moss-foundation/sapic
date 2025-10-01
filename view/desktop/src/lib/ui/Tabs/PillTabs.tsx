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
    <TabsPrimitive.TabsList className={cn("w-full", className)} data-tabs-list-container>
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
      //prettier-ignore
      className={cn(`
        p-0 border-none! text-base leading-4
        cursor-pointer
        rounded-full 

        has-[:focus-visible]:outline-3 
        has-[:focus-visible]:outline-offset-1 
        has-[:focus-visible]:outline-(--moss-primary) 

        ring ring-(--moss-border-color)

        hover:not-data-[state=active]:hover:ring-(--moss-secondary-background-hover) 

        data-[state=active]:ring-2 
        data-[state=active]:ring-offset-0
        data-[state=active]:ring-(--moss-primary)

        disabled:data-[state=active]:ring-(--moss-gray-11)
      `,
        className
      )}
      disabled={disabled}
    >
      <div
        className={cn("flex cursor-pointer items-center gap-[5px] py-2", {
          "px-2": leadingContent && trailingContent,
          "pr-3 pl-2": leadingContent && !trailingContent,
          "pr-2 pl-3": !leadingContent && trailingContent,
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
