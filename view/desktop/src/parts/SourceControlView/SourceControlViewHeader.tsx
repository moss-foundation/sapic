import { ActionButton, SidebarHeader } from "@/components";

export const SourceControlViewHeader = () => {
  return (
    <SidebarHeader
      title="Commit"
      actionsContent={
        <>
          <ActionButton title="Refresh" icon="Refresh" />
        </>
      }
    />
  );
};
