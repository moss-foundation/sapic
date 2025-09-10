import { useCallback } from "react";

import { ActionButton, PageToolbar, PageView } from "@/components";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { useStreamCollectionEntries } from "@/hooks";
import { useRequestPage } from "@/pages/RequestPage/hooks/useRequestPage";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { RequestInputField } from "./RequestInputField";
import { RequestPageHeader } from "./RequestPageHeader/RequestPageHeader";
import { RequestPageTabs } from "./RequestPageTabs";
import { areUrlsEquivalent, parseUrl } from "./utils/urlParser";

export interface RequestPageProps {
  node: TreeCollectionNode;
  collectionId: string;
  iconType: EntryKind;
}

const RequestPage = ({ ...props }: IDockviewPanelProps<RequestPageProps>) => {
  const { data: streamedEntries } = useStreamCollectionEntries(props.params?.collectionId);
  const node = streamedEntries?.find((entry) => entry.id === props.params?.node?.id);

  const { requestData, httpMethod, setHttpMethod, updateRequestData } = useRequestPage();

  const dontShowTabs =
    !node ||
    props.params.node.kind === "Dir" ||
    props.params.node.class === "Endpoint" ||
    props.params.node.class === "Schema";

  console.log({
    node,
    kind: props.params.node.kind,
    class: props.params.node.class,
    dontShowTabs,
  });

  const toolbar = (
    <PageToolbar>
      <ActionButton icon="MoreHorizontal" />
    </PageToolbar>
  );

  const handleSendRequest = (method: string, url: string) => {
    console.log("Sending request:", { method, url });
    // TODO: Implement actual request sending logic
    // Use getRequestUrlWithPathValues() for backend requests with actual path values
  };

  const handleUrlChange = useCallback(
    (url: string) => {
      // Prevent unnecessary updates if URLs are functionally equivalent
      if (areUrlsEquivalent(url, requestData.url.raw)) {
        return;
      }

      const parsed = parseUrl(url);

      const updatedData = {
        url: {
          raw: url,
          originalPathTemplate: parsed.url.originalPathTemplate,
          port: parsed.url.port,
          host: parsed.url.host,
          path_params: parsed.url.path_params,
          query_params: parsed.url.query_params,
        },
      };
      updateRequestData(updatedData);
    },
    [requestData.url.raw, updateRequestData]
  );

  return (
    <PageView>
      {node && <RequestPageHeader node={node} collectionId={props.params?.collectionId ?? ""} api={props.api} />}

      <div>
        {node ? (
          <div>
            <PageWrapper>
              <RequestInputField
                initialMethod={httpMethod}
                initialUrl={requestData.url.raw}
                onSend={handleSendRequest}
                onUrlChange={handleUrlChange}
                onMethodChange={(method) => {
                  if (method !== httpMethod) {
                    setHttpMethod(method);
                  }
                }}
              />
            </PageWrapper>

            {!dontShowTabs && <RequestPageTabs {...props} />}
          </div>
        ) : (
          <div className="flex flex-1 items-center justify-center">
            <div className="text-center">
              <p className="mb-4 text-sm text-(--moss-secondary-text)">No request selected</p>
            </div>
          </div>
        )}
      </div>
    </PageView>
  );
};

export { RequestPage };
