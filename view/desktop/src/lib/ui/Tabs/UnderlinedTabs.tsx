import { ReactNode } from "react";

import { TabsPrimitive } from "@/lib/ui";
import { cn } from "@/utils";

interface PaddedTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: PaddedTabsProps) => {
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

interface PaddedTabsListProps {
  children: ReactNode;
  className?: string;
}

const List = ({ children, className }: PaddedTabsListProps) => {
  return (
    <TabsPrimitive.TabsList className={cn("flex w-full items-center gap-1", className)} data-tabs-list-container>
      {children}
    </TabsPrimitive.TabsList>
  );
};

interface PaddedTabProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Trigger = ({ value, children, className }: PaddedTabProps) => {
  return (
    <TabsPrimitive.TabsTrigger
      value={value}
      //prettier-ignore
      className={cn(`
         group relative 
         flex items-center min-w-0

         text-base leading-4
         p-3 border-0 
         cursor-pointer truncate

         transition-colors

         text-(--moss-secondary-foreground) hover:text-(--moss-primary-foreground)
         
         data-[state=active]:text-(--moss-primary-foreground)
      `,
        className
      )}
    >
      {children}

      <div
        //prettier-ignore
        className={`
          absolute right-0 bottom-0 left-0 
          h-[2px] w-full rounded-full 
          transition-[opacity,background-color]

          opacity-0 

          group-hover:background-(--moss-secondary-background-hover)
          group-hover:opacity-100
          group-data-[state=active]:opacity-100 
          group-data-[state=active]:background-(--moss-accent) 
        `}
      />
    </TabsPrimitive.TabsTrigger>
  );
};

interface PaddedTabContentProps {
  value: string;
  children: ReactNode;
  className?: string;
}

const Content = ({ value, children, className }: PaddedTabContentProps) => {
  return (
    <TabsPrimitive.TabsContent value={value} className={cn(className)}>
      {children}
    </TabsPrimitive.TabsContent>
  );
};

const UnderlinedTabs = {
  Root,
  List,
  Trigger,
  Content,
};

export default UnderlinedTabs;
