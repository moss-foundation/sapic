import { cn } from "@/utils/cn";

interface WorkspacesListItemIndicatorProps {
  isActive?: boolean;
}

export const WorkspacesListItemIndicator = ({ isActive }: WorkspacesListItemIndicatorProps) => {
  return (
    <div
      className={cn(
        "group-hover/WorkspaceListItem:background-(--moss-gray-12) absolute top-0 left-0 z-0 h-full w-full",
        {
          "background-(--moss-gray-12) border-l border-(--moss-primary)": isActive,
        }
      )}
    />
  );
};
