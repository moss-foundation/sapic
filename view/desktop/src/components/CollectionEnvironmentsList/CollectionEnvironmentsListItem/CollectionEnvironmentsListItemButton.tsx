import { Icon } from "@/lib/ui";

interface CollectionEnvironmentsListItemButtonProps {
  label: string;
}

export const CollectionEnvironmentsListItemButton = ({ label }: CollectionEnvironmentsListItemButtonProps) => {
  return (
    <button className="z-10 flex cursor-pointer items-center gap-2 overflow-hidden">
      <Icon icon="CollectionEnvironment" />
      <div className="truncate">{label}</div>

      <div className="text-(--moss-secondary-text)">(15)</div>
    </button>
  );
};
