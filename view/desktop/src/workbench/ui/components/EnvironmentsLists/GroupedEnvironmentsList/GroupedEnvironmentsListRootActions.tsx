import { ActionButton, ActionMenu } from "@/workbench/ui/components";

export const GroupedEnvironmentsListRootActions = () => {
  return (
    <div className="z-10 flex items-center gap-2">
      <div className="transition-discrete hidden items-center opacity-0 transition-[display,opacity] duration-100 group-hover/GroupedEnvironmentsListRoot:flex group-hover/GroupedEnvironmentsListRoot:opacity-100">
        <ActionButton
          icon="Add"
          onClick={(e) => {
            e.stopPropagation();
          }}
          hoverVariant="list"
        />
      </div>

      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <ActionButton icon="MoreHorizontal" hoverVariant="list" />
        </ActionMenu.Trigger>
        <ActionMenu.Portal>
          <ActionMenu.Content>
            <ActionMenu.Item>1</ActionMenu.Item>
            <ActionMenu.Item>2</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    </div>
  );
};
