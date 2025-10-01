import { ReactNode } from "react";

import { Icon, Icons, Scrollbar, Tabs, TabsContent, TabsList, TabsTrigger } from "@/lib/ui";
import { cn } from "@/utils";

interface PageContainerTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: PageContainerTabsProps) => {
  return (
    <div className="flex grow flex-col overflow-hidden rounded-md border border-(--moss-border-color)">
      <Tabs value={value} onValueChange={onValueChange} className={cn("flex flex-col", className)}>
        <div className="flex flex-1 flex-col">{children}</div>
      </Tabs>
    </div>
  );
};

interface PageContainerTabsListProps {
  children: ReactNode;
  className?: string;
  toolbar?: ReactNode;
}

const List = ({ children, className, toolbar }: PageContainerTabsListProps) => {
  return (
    <Scrollbar
      className={cn(
        "background-(--moss-secondary-background) h-auto w-full min-w-0 items-center justify-between border-none shadow-[0px_-1px_0px_0px_var(--moss-border-color)_inset]",
        { "pr-2": toolbar },
        className
      )}
      classNames={{
        contentEl: "flex justify-between gap-1",
        contentWrapper: "mr-2",
      }}
      data-tabs-list-container
    >
      <TabsList className="flex grow items-center">{children}</TabsList>
      {toolbar && <div className="flex shrink-0 items-center">{toolbar}</div>}
    </Scrollbar>
  );
};

interface PageContainerTabProps {
  value: string;
  children: ReactNode;
  className?: string;
  icon?: Icons;
  count?: number;
}

const Trigger = ({ value, children, className, icon, count }: PageContainerTabProps) => {
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
      {icon && <Icon icon={icon} className="h-4 w-4" />}
      <span className="leading-4">{children}</span>
      {count && (
        <span className="background-(--moss-primary) flex size-4 items-center justify-center rounded-full text-xs leading-2.5 text-white">
          {count}
        </span>
      )}
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
