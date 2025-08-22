import { useCallback, useMemo, useState } from "react";

import {
  ActionButton,
  Breadcrumbs,
  PageContainerWithTabs,
  PageHeader,
  PageTabs,
  PageToolbar,
  PageView,
  TabItem,
} from "@/components";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { useStreamedCollectionEntries } from "@/hooks";
import { useRenameEntryForm } from "@/hooks/useRenameEntryForm";
import { Icon } from "@/lib/ui";
import { useRequestPage } from "@/pages/RequestPage/hooks/useRequestPage";
import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { RequestInputField } from "./RequestInputField";
import {
  AuthTabContent,
  BodyTabContent,
  HeadersTabContent,
  ParamsTabContent,
  PostRequestTabContent,
  PreRequestTabContent,
} from "./tabs";
import { areUrlsEquivalent, parseUrl } from "./utils/urlParser";

const Badge = ({ count }: { count: number }) => (
  <span className="background-(--moss-tab-badge-color) inline-flex h-3.5 w-3.5 min-w-[14px] items-center justify-center rounded-full text-xs leading-none text-(--moss-tab-badge-text)">
    <span className="relative top-[0.5px]">{count}</span>
  </span>
);

interface RequestPageProps {
  node: TreeCollectionNode;
  collectionId: string;
  iconType: EntryKind;
}

const RequestPage = ({ ...props }: IDockviewPanelProps<RequestPageProps>) => {
  const { displayMode } = useRequestModeStore();

  const { data: streamedEntries } = useStreamedCollectionEntries(props.params?.collectionId);
  const node = streamedEntries?.find((entry) => entry.id === props.params?.node?.id);

  const showEndpoint = displayMode === "DESIGN_FIRST" && node?.class === "Endpoint";
  let dontShowTabs = true;

  const [activeTab, setActiveTab] = useState(showEndpoint ? "endpoint" : "request");
  const [activeRequestTabId, setActiveRequestTabId] = useState("params");

  const { requestData, httpMethod, setHttpMethod, updateRequestData } = useRequestPage();

  const { isRenamingEntry, setIsRenamingEntry, handleRenamingEntrySubmit, handleRenamingEntryCancel } =
    useRenameEntryForm(props?.params?.node, props?.params?.collectionId);

  if (node) {
    dontShowTabs =
      props.params.node.kind === "Dir" ||
      props.params.node.class === "Endpoint" ||
      props.params.node.class === "Schema";
  }

  const tabs = (
    <PageTabs>
      {showEndpoint && (
        <button data-active={activeTab === "endpoint"} onClick={() => setActiveTab("endpoint")}>
          Endpoint
        </button>
      )}
      <button data-active={activeTab === "request"} onClick={() => setActiveTab("request")}>
        Request
      </button>
      <button data-active={activeTab === "mock"} onClick={() => setActiveTab("mock")}>
        Mock
      </button>
    </PageTabs>
  );

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

  const paramsCount = useMemo(() => {
    const queryParamsCount = requestData.url.query_params.filter(
      (param) => (param.key.trim() !== "" || param.value.trim() !== "") && !param.disabled
    ).length;
    const pathParamsCount = requestData.url.path_params.filter(
      (param) => param.key.trim() !== "" && !param.disabled
    ).length;
    return queryParamsCount + pathParamsCount;
  }, [requestData.url.query_params, requestData.url.path_params]);

  const requestTabs: TabItem[] = [
    {
      id: "params",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="SquareBrackets" className="h-4 w-4" />
          <span>Params</span>
          <Badge count={paramsCount} />
        </div>
      ),
      content: <ParamsTabContent {...props} />,
    },
    {
      id: "auth",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Auth" className="h-4 w-4" />
          <span>Auth</span>
        </div>
      ),
      content: <AuthTabContent {...props} />,
    },
    {
      id: "headers",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Headers" className="h-4 w-4" />
          <span>Headers</span>
          <Badge count={3} />
        </div>
      ),
      content: <HeadersTabContent {...props} />,
    },
    {
      id: "body",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Braces" className="h-4 w-4" />
          <span>Body</span>
        </div>
      ),
      content: <BodyTabContent {...props} />,
    },
    {
      id: "pre-request",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="PreRequest" className="h-4 w-4" />
          <span>Pre Request</span>
        </div>
      ),
      content: <PreRequestTabContent {...props} />,
    },
    {
      id: "post-request",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="PostRequest" className="h-4 w-4" />
          <span>Post Request</span>
        </div>
      ),
      content: <PostRequestTabContent {...props} />,
    },
  ];

  return (
    <PageView>
      <PageHeader
        icon="Request"
        tabs={dontShowTabs ? null : tabs}
        toolbar={toolbar}
        title={node?.name}
        onTitleChange={handleRenamingEntrySubmit}
        disableTitleChange={false}
        isRenamingTitle={isRenamingEntry}
        setIsRenamingTitle={setIsRenamingEntry}
        handleRenamingFormCancel={handleRenamingEntryCancel}
        {...props}
      />

      <div className={cn("relative")}>
        {node ? (
          <div className="flex shrink-0 flex-col gap-1.5 pt-1.5">
            {props.params?.collectionId && node?.id && (
              <div className="px-5">
                <Breadcrumbs collectionId={props.params.collectionId} nodeId={props.params.node.id} />
              </div>
            )}

            <div className="px-5">
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
            </div>

            {activeTab === "request" && (
              <PageContainerWithTabs
                tabs={requestTabs}
                activeTabId={activeRequestTabId}
                onTabChange={setActiveRequestTabId}
              />
            )}
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
