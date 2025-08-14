import { cn } from "@/utils";

interface GroupedEnvironmentsListItemIndicatorProps {
  isActive?: boolean;
}

export const GroupedEnvironmentsListItemIndicator = ({ isActive }: GroupedEnvironmentsListItemIndicatorProps) => {
  return (
    //prettier-ignore
    <div className={cn(`
      absolute top-0 left-0 
      h-full w-full
      group-hover/GroupedEnvironmentsListItem:background-(--moss-secondary-background-hover) 
      group-hover/GroupedEnvironmentsListRootHeader:background-(--moss-secondary-background-hover) 
    `, {
        "background-(--moss-secondary-background-hover) border-l border-(--moss-primary)": isActive,
      }
    )}
    />
  );
};
