import { cn } from "@/utils/cn";

interface WorkspacesListItemIndicatorProps {
  isActive?: boolean;
}

export const WorkspacesListItemIndicator = ({ isActive }: WorkspacesListItemIndicatorProps) => {
  return (
    <div
      className={cn(
        "group-hover/WorkspaceListItem:background-(--moss-secondary-background-hover) absolute top-0 left-0 z-0 h-full w-full",
        {
          "background-(--moss-secondary-background-hover) border-l border-(--moss-primary)": isActive,
        }
      )}
    />
  );
};
