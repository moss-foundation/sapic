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
        "flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium transition-colors",
        "border-b-2 border-transparent",
        "text-(--moss-secondary-text) hover:text-(--moss-primary-text)",
        "data-[state=active]:border-(--moss-info-background) data-[state=active]:text-(--moss-primary-text)",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-(--moss-info-background)",
        "bg-transparent data-[state=active]:bg-transparent",
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
