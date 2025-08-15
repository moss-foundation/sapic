import { ReactNode } from "react";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/lib/ui";
import { cn } from "@/utils";

import { ProviderIcon } from "../ProviderIcon";

interface ProviderTabsProps {
  value: string;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: ProviderTabsProps) => {
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

interface ProviderTabsListProps {
  children: ReactNode;
  className?: string;
}

const List = ({ children, className }: ProviderTabsListProps) => {
  return (
    <TabsList className={cn("h-full w-full", className)} data-tabs-list-container>
      {children}
    </TabsList>
  );
};

interface PaddedTabProps {
  value: string;
  className?: string;
  icon: ProviderIcon;
  label: string;
}

const Trigger = ({ value, className, icon, label }: PaddedTabProps) => {
  return (
    <TabsTrigger
      value={value}
      //prettier-ignore
      className={cn(`
        has-[:focus-visible]:outline-3 
        has-[:focus-visible]:outline-offset-1 
        has-[:focus-visible]:outline-(--moss-primary) 

        rounded-full 
        border border-(--moss-border-color)

        hover:not-data-[state=active]:hover:border-black 

        data-[state=active]:ring-2 
        data-[state=active]:ring-offset-0
        data-[state=active]:ring-(--moss-primary) 
      `,
        className
      )}
    >
      <div className="flex items-center gap-2">
        <ProviderIcon icon={icon} />
        <span>{label}</span>
      </div>
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

const ProviderTabs = {
  Root,
  List,
  Trigger,
  Content,
};

export default ProviderTabs;
