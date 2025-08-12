import { cn } from "@/utils";

interface CollectionsListItemIndicatorProps {
  isActive?: boolean;
}

export const CollectionsListItemIndicator = ({ isActive }: CollectionsListItemIndicatorProps) => {
  return (
    //prettier-ignore
    <div className={cn(`
      absolute top-0 left-0 
      h-full w-full
      group-hover/CollectionsListItem:background-(--moss-secondary-background-hover) 
      group-hover/CollectionsListRootHeader:background-(--moss-secondary-background-hover) 
    `, {
        "background-(--moss-secondary-background-hover) border-l border-(--moss-primary)": isActive,
      }
    )}
    />
  );
};
