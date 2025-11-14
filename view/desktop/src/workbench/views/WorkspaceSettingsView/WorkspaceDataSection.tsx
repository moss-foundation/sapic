import { Button } from "@/lib/ui";

import { SectionTitle } from "./SectionTitle";

export const WorkspaceDataSection = () => {
  return (
    <div className="text-(--moss-primary-foreground) mt-8">
      <SectionTitle>Data and Storage</SectionTitle>
      <div className="flex w-[36rem] items-center justify-between">
        <div>
          <p className="mb-1 font-medium">Delete this workspace</p>
          <p className="text-(--moss-secondary-foreground) text-sm">Last checked: 25 MB</p>
        </div>
        <Button intent="outlined" disabled>
          Clear
        </Button>
      </div>
    </div>
  );
};
