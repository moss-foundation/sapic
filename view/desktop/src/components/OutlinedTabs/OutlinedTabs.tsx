import { ReactNode } from "react";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/lib/ui";
import { cn } from "@/utils";

interface OutlinedTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: OutlinedTabsProps) => {
  return (
    <Tabs
      value={value}
      onValueChange={onValueChange}
      className={cn("flex h-full min-h-fit min-w-fit flex-col", className)}
    >
      {children}
    </Tabs>
  );
};

interface OutlinedTabsListProps {
  children: ReactNode;
  className?: string;
}
const List = ({ children, className }: OutlinedTabsListProps) => {
  return (
    <TabsList
      className={cn("flex w-full items-center gap-1 border-b border-(--moss-border-color) px-5", className)}
      data-tabs-list-container
    >
      {children}
    </TabsList>
  );
};

interface OutlinedTabProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Trigger = ({ value, children, className }: OutlinedTabProps) => {
  return (
    <TabsTrigger
      value={value}
      //prettier-ignore
      className={cn(`
         group relative 
         flex items-center min-w-0

         text-base leading-5
         px-4 py-1 
         cursor-pointer truncate

         transition-colors

         text-(--moss-secondary-text) hover:text-(--moss-primary-text)

         border-b-0
         border-t-1
         border-l-1
         border-r-1
         
         data-[state=active]:text-(--moss-primary-text)

         data-[state=active]:rounded-tl-md 
         data-[state=active]:rounded-tr-md

         data-[state=active]:border-t-(--moss-border-color)
         data-[state=active]:border-l-(--moss-border-color)
         data-[state=active]:border-r-(--moss-border-color)

         data-[state=active]:shadow-[0px_1px_0px_0px_var(--moss-primary-background)]
      `,
        className
      )}
    >
      {children}
    </TabsTrigger>
  );
};

interface OutlinedTabContentProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Content = ({ value, children, className }: OutlinedTabContentProps) => {
  return (
    <TabsContent value={value} className={cn(className)}>
      {children}
    </TabsContent>
  );
};

const OutlinedTabs = {
  Root,
  List,
  Trigger,
  Content,
};

export default OutlinedTabs;
