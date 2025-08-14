import { ActionButton, ActionMenu } from "@/components";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

interface GroupedEnvironmentsListItemActionsProps {
  showActionMenu: boolean;
  setShowActionMenu: (showActionMenu: boolean) => void;
}

export const GroupedEnvironmentsListItemActions = ({
  showActionMenu,
  setShowActionMenu,
}: GroupedEnvironmentsListItemActionsProps) => {
  return (
    <div
      className={cn("sr-only z-10 flex items-center gap-2 group-hover/GroupedEnvironmentsListItem:not-sr-only", {
        "not-sr-only": showActionMenu,
      })}
    >
      <Icon icon="EnvironmentSelection" />
      <ActionMenu.Root onOpenChange={setShowActionMenu} modal={showActionMenu}>
        <ActionMenu.Trigger asChild>
          <ActionButton icon="MoreHorizontal" customHoverBackground="hover:background-(--moss-gray-10)" />
        </ActionMenu.Trigger>
        <ActionMenu.Portal>
          <ActionMenu.Content>
            <ActionMenu.Item>Edit</ActionMenu.Item>
            <ActionMenu.Item>Delete</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    </div>
  );
};
