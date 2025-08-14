import { ReactNode } from "react";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/lib/ui";
import { cn } from "@/utils";

interface PaddedTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: PaddedTabsProps) => {
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

interface PaddedTabsListProps {
  children: ReactNode;
  className?: string;
}

const List = ({ children, className }: PaddedTabsListProps) => {
  return (
    <TabsList className={cn("flex h-full w-full items-center", className)} data-tabs-list-container>
      {children}
    </TabsList>
  );
};

interface PaddedTabProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Trigger = ({ value, children, className }: PaddedTabProps) => {
  return (
    <TabsTrigger
      value={value}
      //prettier-ignore
      className={cn(`
         group
         relative 
         flex items-center 

         min-w-0 flex-shrink-0
         p-3 leading-4 border-0
         cursor-pointer truncate

         transition-colors

         text-(--moss-secondary-text) hover:text-(--moss-primary-text)
         
         data-[state=active]:text-(--moss-primary-text)
      `,
        className
      )}
    >
      {children}

      <div
        //prettier-ignore
        className={`
          absolute right-0 bottom-0 left-0 
          h-[3px] w-full rounded-full 
          transition-opacity
          
          background-(--moss-tab-active-border-color)
          opacity-0 
          group-data-[state=active]:opacity-100 
          group-hover:opacity-50
        `}
      />
    </TabsTrigger>
  );
};

interface PaddedTabContentProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Content = ({ value, children, className }: PaddedTabContentProps) => {
  return (
    <TabsContent value={value} className={cn(className)}>
      {children}
    </TabsContent>
  );
};

const PaddedTabs = {
  Root,
  List,
  Trigger,
  Content,
};

export default PaddedTabs;
