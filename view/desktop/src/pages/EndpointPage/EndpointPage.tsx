import { PageView } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useStreamProjectEntries } from "@/hooks";
import { EntryKind } from "@repo/moss-project";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { EndpointPageBody, EndpointPageHeader } from "./components";

export interface EndpointPageProps {
  node: ProjectTreeNode;
  projectId: string;
  iconType: EntryKind;
}

const EndpointPage = ({ ...props }: IDockviewPanelProps<EndpointPageProps>) => {
  const { data: streamedEntries } = useStreamProjectEntries(props.params?.projectId);
  const node = streamedEntries?.find((entry) => entry.id === props.params?.node?.id);

  if (!node) {
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
    <PageView>
      <EndpointPageHeader node={node} projectId={props.params?.projectId ?? ""} api={props.api} />
      <EndpointPageBody {...props} />
    </PageView>
  );
};

export { EndpointPage };
