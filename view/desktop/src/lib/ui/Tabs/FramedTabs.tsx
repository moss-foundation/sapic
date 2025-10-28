import { ReactNode } from "react";

import { TabsPrimitive } from "@/lib/ui";
import { cn } from "@/utils";

interface FramedTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: FramedTabsProps) => {
  return (
    <TabsPrimitive.Tabs
      value={value}
      onValueChange={onValueChange}
      className={cn("flex min-h-fit grow flex-col", className)}
    >
      {children}
    </TabsPrimitive.Tabs>
  );
};

interface FramedListProps {
  children: ReactNode;
  className?: string;
}

const List = ({ children, className }: FramedListProps) => {
  return (
    <TabsPrimitive.TabsList
      className={cn("border-(--moss-border) flex w-full items-center gap-1 border-b px-5", className)}
      data-tabs-list-container
    >
      {children}
    </TabsPrimitive.TabsList>
  );
};

interface FramedTabProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Trigger = ({ value, children, className }: FramedTabProps) => {
  return (
    <TabsPrimitive.TabsTrigger
      value={value}
      className={cn(
        "group relative",
        "flex min-w-0 items-center",
        "px-4 py-1",
        "text-base leading-5",
        "cursor-pointer truncate",
        "transition-colors",
        "text-(--moss-secondary-foreground) hover:text-(--moss-primary-foreground)",
        "border-t-1 border-r-1 border-l-1 rounded-tl-md rounded-tr-md border-b-0",
        "data-[state=active]:text-(--moss-primary-foreground)",
        "data-[state=active]:border-t-(--moss-border) data-[state=active]:border-r-(--moss-border) data-[state=active]:border-l-(--moss-border)",
        "data-[state=active]:shadow-[0px_1px_0px_0px_var(--moss-primary-background)]",
        className
      )}
    >
      <div className="group-hover:background-(--moss-secondary-background-hover) absolute left-[10%] top-[10%] h-[80%] w-[80%] rounded-md px-4 py-1 transition-colors group-data-[state=active]:hidden" />
      <div className="z-10">{children}</div>
    </TabsPrimitive.TabsTrigger>
  );
};

interface FramedTabContentProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Content = ({ value, children, className }: FramedTabContentProps) => {
  return (
    <TabsPrimitive.TabsContent value={value} className={cn(className)}>
      {children}
    </TabsPrimitive.TabsContent>
  );
};

const FramedTabs = {
  Root,
  List,
  Trigger,
  Content,
};

export default FramedTabs;
