import { Icon } from "@/lib/ui";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface WorkspaceListItemButtonProps {
  environment: StreamEnvironmentsEvent;
  onClick: () => void;
}

export const WorkspaceListItemButton = ({ environment, onClick }: WorkspaceListItemButtonProps) => {
  return (
    <button className="z-10 flex cursor-pointer items-center gap-2 overflow-hidden" onClick={onClick}>
      <Icon icon="Environment" />
      <span className="truncate">{environment.name}</span>
    </button>
  );
};
