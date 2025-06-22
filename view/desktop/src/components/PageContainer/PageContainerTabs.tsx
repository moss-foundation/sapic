import React from "react";
import { cn } from "@/utils";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/lib/ui";

interface PageContainerTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: React.ReactNode;
  className?: string;
}

export const PageContainerTabs: React.FC<PageContainerTabsProps> = ({ value, onValueChange, children, className }) => {
  return (
    <Tabs value={value} onValueChange={onValueChange} className={cn("flex h-full flex-col", className)}>
      {children}
    </Tabs>
  );
};

interface PageContainerTabsListProps {
  children: React.ReactNode;
  className?: string;
}

export const PageContainerTabsList: React.FC<PageContainerTabsListProps> = ({ children, className }) => {
  return <TabsList className={cn("flex h-auto items-center bg-transparent p-0", className)}>{children}</TabsList>;
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
        "flex items-center gap-2 px-4 py-2 text-sm font-medium transition-colors",
        "relative border-b border-transparent",
        "text-(--moss-secondary-text) hover:text-(--moss-primary-text)",
        "data-[state=active]:text-(--moss-primary-text)",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-(--moss-primary) focus-visible:ring-offset-2",
        "bg-transparent data-[state=active]:bg-transparent",
        "min-w-0",
        // Active state - creates a 1px border that sits on the header bottom border
        "data-[state=active]:after:absolute data-[state=active]:after:right-0 data-[state=active]:after:bottom-[-1px] data-[state=active]:after:left-0",
        "data-[state=active]:after:h-px data-[state=active]:after:bg-(--moss-tab-active-border-color) data-[state=active]:after:content-['']",
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
    <TabsContent value={value} className={cn("flex-1 overflow-auto", className)}>
      <div className="p-3">{children}</div>
    </TabsContent>
  );
};
