import { PageView } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useStreamCollectionEntries } from "@/hooks";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { RequestPageHeader } from "./RequestPageHeader/RequestPageHeader";
import { RequestPageTabs } from "./RequestPageTabs";

export interface RequestPageProps {
  node: ProjectTreeNode;
  collectionId: string;
  iconType: EntryKind;
}

const RequestPage = ({ ...props }: IDockviewPanelProps<RequestPageProps>) => {
  const { data: streamedEntries } = useStreamCollectionEntries(props.params?.collectionId);
  const node = streamedEntries?.find((entry) => entry.id === props.params?.node?.id);

  return (
    <PageView>
      {node && <RequestPageHeader node={node} collectionId={props.params?.collectionId ?? ""} api={props.api} />}

      {node ? (
        <RequestPageTabs {...props} />
      ) : (
        <PageWrapper>
          <div className="flex flex-1 items-center justify-center">
            <div className="text-center">
              <p className="mb-4 text-sm text-(--moss-secondary-text)">No request selected</p>
            </div>
          </div>
        </PageWrapper>
      )}
    </PageView>
  );
};

export { RequestPage };
