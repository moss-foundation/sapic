import { Icon } from "@/lib/ui";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

interface GlobalEnvironmentsListButtonProps {
  environment: StreamEnvironmentsEvent;
}

export const GlobalEnvironmentsListButton = ({ environment }: GlobalEnvironmentsListButtonProps) => {
  return (
    <button className="z-10 flex cursor-pointer items-center gap-2 overflow-hidden">
      <Icon icon="Environment" />
      <span className="truncate font-medium">{environment.name}</span>
    </button>
  );
};
