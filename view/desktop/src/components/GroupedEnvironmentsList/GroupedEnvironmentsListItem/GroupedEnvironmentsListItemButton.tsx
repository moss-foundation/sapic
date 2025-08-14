import { Icon } from "@/lib/ui";

interface GroupedEnvironmentsListItemButtonProps {
  label: string;
}

export const GroupedEnvironmentsListItemButton = ({ label }: GroupedEnvironmentsListItemButtonProps) => {
  return (
    <button className="z-10 flex cursor-pointer items-center gap-2 overflow-hidden">
      <Icon icon="GroupedEnvironment" />
      <div className="truncate">{label}</div>

      <div className="text-(--moss-secondary-text)">(15)</div>
    </button>
  );
};
