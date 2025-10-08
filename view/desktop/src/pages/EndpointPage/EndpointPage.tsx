import { PageView } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useDescribeProjectEntry, useStreamProjectEntries } from "@/hooks";
import { EntryKind } from "@repo/moss-project";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { EndpointPageBody, EndpointPageHeader } from "./components";
import { EndpointPageContext } from "./EndpointPageContext";

export interface EndpointPageProps {
  node: ProjectTreeNode;
  projectId: string;
  iconType: EntryKind;
}

const EndpointPage = ({ ...props }: IDockviewPanelProps<EndpointPageProps>) => {
  const { data: streamedEntries } = useStreamProjectEntries(props.params?.projectId);
  const entry = streamedEntries?.find((entry) => entry.id === props.params?.node?.id);

  const { data: entryDescription } = useDescribeProjectEntry({
    projectId: props.params?.projectId ?? "",
    entryId: entry?.id ?? "",
  });

  if (!entry || !entryDescription) {
    return (
      <PageWrapper>
        <div className="flex flex-1 items-center justify-center">
          <div className="text-center">
            <p className="mb-4 text-sm text-(--moss-secondary-text)">No endpoint selected</p>
          </div>
        </div>
      </PageWrapper>
    );
  }

  return (
    <EndpointPageContext.Provider value={{ projectId: props.params.projectId, entryDescription, entry: entry }}>
      <PageView>
        <EndpointPageHeader />
        <EndpointPageBody />
      </PageView>
    </EndpointPageContext.Provider>
  );
};

export { EndpointPage };
