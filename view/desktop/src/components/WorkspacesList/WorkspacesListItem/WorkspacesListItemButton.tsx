import { Icon } from "@/lib/ui";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface WorkspacesListItemButtonProps {
  environment: StreamEnvironmentsEvent;
}

export const WorkspacesListItemButton = ({ environment }: WorkspacesListItemButtonProps) => {
  return (
    <button className="z-10 flex cursor-pointer items-center gap-2 overflow-hidden">
      <span className="text-xs text-gray-500 underline">{environment.order}</span>
      <Icon icon="Environment" />
      <span className="truncate">{environment.name}</span>
    </button>
  );
};
