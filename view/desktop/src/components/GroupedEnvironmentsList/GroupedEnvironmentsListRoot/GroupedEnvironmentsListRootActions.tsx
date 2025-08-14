import { ActionButton, ActionMenu } from "@/components";

export const GroupedEnvironmentsListRootActions = () => {
  return (
    <div className="z-10 flex items-center gap-2">
      <div className="hidden items-center opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover/GroupedEnvironmentsListRoot:flex group-hover/GroupedEnvironmentsListRoot:opacity-100">
        <ActionButton
          icon="Add"
          onClick={(e) => {
            e.stopPropagation();
          }}
          customHoverBackground="hover:background-(--moss-gray-10)"
        />
      </div>

      <ActionMenu.Root>
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
