import { ActionButton, ActionMenu } from "@/components";

export const CollectionsListRootActions = () => {
  return (
    <div className="z-10 flex items-center gap-2">
      <div className="hidden items-center opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover/CollectionsListRoot:flex group-hover/CollectionsListRoot:opacity-100">
        <ActionButton
          icon="Add"
          onClick={(e) => {
            e.stopPropagation();
          }}
        />
      </div>

      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <ActionButton icon="MoreHorizontal" />
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
