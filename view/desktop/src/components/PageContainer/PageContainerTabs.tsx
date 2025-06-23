import React from "react";
import { cn } from "@/utils";
import { Tabs, TabsList, TabsTrigger, TabsContent, Scrollbar } from "@/lib/ui";

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
  const handleWheel = (e: React.WheelEvent<HTMLDivElement>) => {
    const container = e.currentTarget.querySelector("[data-overlayscrollbars-contents]") as HTMLElement;
    if (container) {
      e.preventDefault();
      container.scrollLeft += e.deltaY;
    }
  };

  return (
    <div className={cn("flex h-full w-full", className)} onWheel={handleWheel}>
      <Scrollbar
        className="flex-1 overflow-x-auto overflow-y-hidden"
        options={{
          scrollbars: {
            autoHide: "move",
            autoHideDelay: 1000,
          },
          overflow: {
            x: "scroll",
            y: "hidden",
          },
        }}
      >
        <TabsList className="flex h-full w-max items-center bg-transparent p-0">{children}</TabsList>
      </Scrollbar>
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
        "relative border-b-2 border-transparent",
        "text-(--moss-secondary-text) hover:text-(--moss-primary-text)",
        "data-[state=active]:text-(--moss-primary-text)",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-(--moss-primary) focus-visible:ring-offset-2",
        "bg-transparent data-[state=active]:bg-transparent",
        "min-w-0 flex-shrink-0 whitespace-nowrap",
        // Active state - use direct border instead of pseudo-element
        "data-[state=active]:border-b-[var(--moss-tab-active-border-color)]",
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
      <Scrollbar className="h-full overflow-auto">
        <div className="min-w-fit p-3">{children}</div>
      </Scrollbar>
    </TabsContent>
  );
};
