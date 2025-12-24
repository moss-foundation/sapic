import { ReactNode } from "react";

import { Icon, Icons, Scrollbar, TabsPrimitive } from "@/lib/ui";
import { cn } from "@/utils";

interface FolderTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: FolderTabsProps) => {
  return (
    <div className="border-(--moss-border) flex grow flex-col rounded-md border">
      <TabsPrimitive.Tabs value={value} onValueChange={onValueChange} className={cn("flex flex-col", className)}>
        <div className="flex flex-1 flex-col">{children}</div>
      </TabsPrimitive.Tabs>
    </div>
  );
};

interface FolderTabsListProps {
  children: ReactNode;
  className?: string;
  toolbar?: ReactNode;
}

const List = ({ children, className, toolbar }: FolderTabsListProps) => {
  return (
    <Scrollbar
      className={cn(
        "background-(--moss-secondary-background) h-auto w-full min-w-0 items-center shadow-[0px_-1px_0px_0px_var(--moss-border)_inset]",
        { "pr-2": toolbar }
      )}
      classNames={{
        contentWrapper: "mr-2",
        contentEl: "flex items-center justify-between",
      }}
      data-tabs-list-container
    >
      <TabsPrimitive.TabsList className={cn("flex grow items-center", className)}>{children}</TabsPrimitive.TabsList>
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
    <TabsPrimitive.TabsTrigger
      value={value}
      className={cn(
        "py-2.25 flex items-center text-base transition-colors",
        "border-b-1 relative border-transparent",
        "text-(--moss-secondary-foreground) hover:text-(--moss-primary-foreground)",
        "data-[state=active]:text-(--moss-primary-foreground)",
        "focus-visible:ring-(--moss-accent) focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2",
        "bg-transparent data-[state=active]:bg-transparent",
        "min-w-0 flex-shrink-0 whitespace-nowrap",
        "data-[state=active]:border-b-(--moss-accent)",
        "cursor-pointer",
        className
      )}
    >
      {icon && <Icon icon={icon} className="h-4 w-4" />}
      <span className="leading-4">{children}</span>
      {count !== undefined && (
        <span className="background-(--moss-accent) leading-2.5 flex size-4 items-center justify-center rounded-full text-xs text-white">
          {count}
        </span>
      )}
    </TabsPrimitive.TabsTrigger>
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
    <TabsPrimitive.TabsContent value={value} className={className}>
      {children}
    </TabsPrimitive.TabsContent>
  );
};

const FolderTabs = {
  Root,
  List,
  Trigger,
  Content,
};

export default FolderTabs;
