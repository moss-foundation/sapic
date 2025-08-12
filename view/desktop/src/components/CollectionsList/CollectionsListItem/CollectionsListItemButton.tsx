import { Icon } from "@/lib/ui";

interface CollectionsListItemButtonProps {
  label: string;
}

export const CollectionsListItemButton = ({ label }: CollectionsListItemButtonProps) => {
  return (
    <button className="z-10 flex cursor-pointer items-center gap-2 overflow-hidden">
      <Icon icon="CollectionEnvironment" />
      <div className="truncate">{label}</div>
      <div className="text-(--moss-secondary-text)">({getRandomInt(1, 25)})</div>
    </button>
  );
};

const getRandomInt = (min: number, max: number) => {
  return Math.floor(Math.random() * (max - min + 1)) + min;
};
