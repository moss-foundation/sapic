import { ActionButton } from "@/workbench/ui/components";

import { SidebarHeader } from "../Sidebar";

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
