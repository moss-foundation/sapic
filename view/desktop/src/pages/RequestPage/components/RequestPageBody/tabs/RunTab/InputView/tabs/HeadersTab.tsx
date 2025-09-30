import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { Icon } from "@/lib/ui";
import { EntryKind } from "@repo/moss-project";
import { IDockviewPanelProps } from "@repo/moss-tabs";

export const HeadersTabContent = ({}: IDockviewPanelProps<{
  node?: ProjectTreeNode;
  projectId: string;
  iconType: EntryKind;
}>) => {
  return (
    <div className="flex grow flex-col items-center justify-center gap-4 text-center opacity-60">
      <Icon icon="Headers" className="size-16 text-(--moss-secondary-text)" />

      <div className="flex flex-col items-center justify-center gap-2">
        <h3 className="text-lg font-medium text-(--moss-primary-text)">Headers</h3>
        <p className="text-(--moss-secondary-text)">This section will contain request headers configuration</p>
        <p className="text-sm text-(--moss-secondary-text)">Coming soon...</p>
      </div>
    </div>
  );
};
