import { ActionButton } from "@/workbench/ui/components";

import { SidebarHeader } from "../../SidebarHeader";

export const SourceControlViewHeader = () => {
  return (
    <SidebarHeader
      toolbar={
        <>
          <ActionButton title="Refresh" icon="Refresh" />
        </>
      }
    />
  );
};
