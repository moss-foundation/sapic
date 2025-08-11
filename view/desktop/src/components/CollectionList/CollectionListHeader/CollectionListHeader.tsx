import { ActionButton, ActionMenu } from "@/components";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { StreamCollectionsEvent } from "@repo/moss-workspace";

interface CollectionListHeaderProps {
  collection: StreamCollectionsEvent;
  onToggleChildren: (showChildren: boolean) => void;
  showChildren: boolean;
}

export const CollectionListHeader = ({ collection, onToggleChildren, showChildren }: CollectionListHeaderProps) => {
  return (
    <div className="flex h-[30px] items-center justify-between p-2">
      <div className="flex items-center gap-2">
        <button
          onClick={() => onToggleChildren(!showChildren)}
          className="hover:background-(--moss-icon-primary-background-hover) flex h-4 w-4 cursor-pointer items-center justify-center rounded-full"
        >
          <Icon icon="ChevronRight" className={cn(showChildren && "rotate-90")} />
        </button>

        <div>{collection.name}</div>
      </div>

      <div className="flex items-center gap-2">
        <ActionButton icon="Add" />
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <ActionButton icon="MoreHorizontal" />
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            <ActionMenu.Item>Edit</ActionMenu.Item>
            <ActionMenu.Item>Delete</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Root>
      </div>
    </div>
  );
};
