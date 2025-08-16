import { ReactNode } from "react";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/lib/ui";
import { cn } from "@/utils";

import { ProviderIcon } from "../ProviderIcon";

interface ProviderTabsProps {
  value?: string | null;
  onValueChange: (value: string) => void;
  children: ReactNode;
  className?: string;
}

const Root = ({ value, onValueChange, children, className }: ProviderTabsProps) => {
  return (
    <Tabs
      value={value ?? undefined}
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
  disabled?: boolean;
}

const Trigger = ({ value, className, icon, label, disabled }: PaddedTabProps) => {
  return (
    <TabsTrigger
      value={value}
      //prettier-ignore
      className={cn(`
        p-0 border-none! test-base leading-4
        cursor-pointer
        rounded-full 

        has-[:focus-visible]:outline-3 
        has-[:focus-visible]:outline-offset-1 
        has-[:focus-visible]:outline-(--moss-primary) 

        ring ring-(--moss-border-color)

        hover:not-data-[state=active]:hover:ring-(--moss-secondary-background-hover) 

        data-[state=active]:ring-2 
        data-[state=active]:ring-offset-0
        data-[state=active]:ring-(--moss-primary)

        disabled:data-[state=active]:ring-(--moss-gray-11)
      `,
        className
      )}
      disabled={disabled}
    >
      <div className="flex cursor-pointer items-center gap-[5px] py-2 pr-3 pl-2">
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
