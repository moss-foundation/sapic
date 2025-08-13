import { cn } from "@/utils";

interface CollectionEnvironmentsListItemIndicatorProps {
  isActive?: boolean;
}

export const CollectionEnvironmentsListItemIndicator = ({ isActive }: CollectionEnvironmentsListItemIndicatorProps) => {
  return (
    //prettier-ignore
    <div className={cn(`
      absolute top-0 left-0 
      h-full w-full
      group-hover/CollectionEnvironmentsListItem:background-(--moss-secondary-background-hover) 
      group-hover/CollectionEnvironmentsListRootHeader:background-(--moss-secondary-background-hover) 
    `, {
        "background-(--moss-secondary-background-hover) border-l border-(--moss-primary)": isActive,
      }
    )}
    />
  );
};
