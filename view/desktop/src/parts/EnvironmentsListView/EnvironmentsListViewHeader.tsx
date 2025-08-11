import { ActionButton, SidebarHeader } from "@/components";

export const EnvironmentsListViewHeader = () => {
  return (
    <SidebarHeader
      title="Environments"
      actionsContent={
        <>
          <ActionButton icon="Add" />
          <ActionButton icon="Import" />
          <ActionButton icon="Refresh" />
        </>
      }
    />
  );
};
