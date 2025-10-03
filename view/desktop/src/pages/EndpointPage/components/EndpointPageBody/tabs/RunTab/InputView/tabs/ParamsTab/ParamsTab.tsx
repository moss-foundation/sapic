import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { Resizable, ResizablePanel } from "@/lib/ui";
import { EntryKind } from "@repo/moss-project";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { PathParamsView } from "./components/PathParamsView/PathParamsView";
import { QueryParamsView } from "./components/QueryParamsView/QueryParamsView";

export const ParamsTabContent = ({
  ...props
}: IDockviewPanelProps<{
  node?: ProjectTreeNode;
  projectId: string;
  iconType: EntryKind;
}>) => {
  return (
    <Resizable vertical>
      <ResizablePanel minSize={32}>
        <QueryParamsView />
      </ResizablePanel>
      <ResizablePanel minSize={32}>
        <PathParamsView />
      </ResizablePanel>
    </Resizable>
  );
};
