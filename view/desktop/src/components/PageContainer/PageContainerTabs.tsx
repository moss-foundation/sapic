import React from "react";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/lib/ui";
import { cn } from "@/utils";

interface PageContainerTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: React.ReactNode;
  className?: string;
}

export const PageContainerTabs: React.FC<PageContainerTabsProps> = ({ value, onValueChange, children, className }) => {
  return (
    <Tabs value={value} onValueChange={onValueChange} className={cn("flex h-full flex-col", className)}>
      <div className="flex h-full min-h-fit min-w-fit flex-col">{children}</div>
    </Tabs>
  );
};

interface PageContainerTabsListProps {
  children: React.ReactNode;
  className?: string;
}

export const PageContainerTabsList: React.FC<PageContainerTabsListProps> = ({ children, className }) => {
  return (
    <div className={cn("flex h-full w-full min-w-0", className)} data-tabs-list-container>
      <TabsList className="flex h-full w-max items-center bg-transparent p-0">{children}</TabsList>
    </div>
  );
};

interface PageContainerTabProps {
  value: string;
  children: React.ReactNode;
  className?: string;
}

export const PageContainerTab: React.FC<PageContainerTabProps> = ({ value, children, className }) => {
  return (
    <TabsTrigger
      value={value}
      className={cn(
        "mr-2 flex items-center px-2.5 py-2 text-base transition-colors",
        "relative border-b-1 border-transparent",
        "text-(--moss-secondary-text) hover:text-(--moss-primary-text)",
        "data-[state=active]:text-(--moss-primary-text)",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-(--moss-primary) focus-visible:ring-offset-2",
        "bg-transparent data-[state=active]:bg-transparent",
        "min-w-0 flex-shrink-0 whitespace-nowrap",
        // Active state - use direct border instead of pseudo-element
        "data-[state=active]:border-b-(--moss-tab-active-border-color)",
        // Cursor styling
        "cursor-pointer",

        className
      )}
    >
      {children}
    </TabsTrigger>
  );
};

interface PageContainerTabContentProps {
  value: string;
  children: React.ReactNode;
  className?: string;
}

export const PageContainerTabContent: React.FC<PageContainerTabContentProps> = ({ value, children, className }) => {
  return (
    <TabsContent value={value} className={cn("flex-1", className)}>
      <div className="h-full min-w-fit p-3">{children}</div>
    </TabsContent>
  );
};
