import { ReactNode } from "react";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/lib/ui";
import { cn } from "@/utils";

interface PageContainerTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: PageContainerTabsProps) => {
  return (
    <Tabs
      value={value}
      onValueChange={onValueChange}
      className={cn("flex h-full flex-col rounded-md border border-(--moss-border-color)", className)}
    >
      <div className="flex h-full min-h-fit min-w-fit flex-col">{children}</div>
    </Tabs>
  );
};

interface PageContainerTabsListProps {
  children: ReactNode;
  className?: string;
}

const List = ({ children, className }: PageContainerTabsListProps) => {
  return (
    <div
      className={cn(
        "background-(--moss-secondary-background) flex h-full w-full min-w-0 border-b border-(--moss-border-color)",
        className
      )}
      data-tabs-list-container
    >
      <TabsList className="flex h-full w-max items-center p-0">{children}</TabsList>
    </div>
  );
};

interface PageContainerTabProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Trigger = ({ value, children, className }: PageContainerTabProps) => {
  return (
    <TabsTrigger
      value={value}
      className={cn(
        "flex items-center py-2 text-base transition-colors",
        "relative border-b-1 border-transparent",
        "text-(--moss-secondary-text) hover:text-(--moss-primary-text)",
        "data-[state=active]:text-(--moss-primary-text)",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-(--moss-primary) focus-visible:ring-offset-2",
        "bg-transparent data-[state=active]:bg-transparent",
        "min-w-0 flex-shrink-0 whitespace-nowrap",
        "data-[state=active]:border-b-(--moss-tab-active-border-color)",
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
  children: ReactNode;
  className?: string;
  noPadding?: boolean;
}

const Content = ({ value, children, className }: PageContainerTabContentProps) => {
  return (
    <TabsContent value={value} className={className}>
      {children}
    </TabsContent>
  );
};

const PageContainerTabs = {
  Root,
  List,
  Trigger,
  Content,
};

export default PageContainerTabs;
