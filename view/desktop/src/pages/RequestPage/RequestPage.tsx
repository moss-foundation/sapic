import React from "react";

import {
  ActionButton,
  Breadcrumbs,
  PageContent,
  PageContainerWithTabs,
  PageHeader,
  PageTabs,
  PageToolbar,
  PageView,
  TabItem,
} from "@/components";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { Icon } from "@/lib/ui";
import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import Metadata from "../../parts/TabbedPane/DebugComponents/Metadata";
import { RequestInputField } from "./RequestInputField";
import {
  AuthTabContent,
  BodyTabContent,
  HeadersTabContent,
  ParamsTabContent,
  PostRequestTabContent,
  PreRequestTabContent,
} from "./tabs";
import { parseUrl } from "./utils/urlParser";
import { useRequestPageStore } from "@/store/requestPage";

const DebugContext = React.createContext<boolean>(false);

const Badge = ({ count }: { count: number }) => (
  <span className="background-(--moss-tab-badge-color) inline-flex h-3.5 w-3.5 min-w-[14px] items-center justify-center rounded-full text-xs leading-none font-medium text-(--moss-tab-badge-text)">
    <span className="relative top-[0.5px]">{count}</span>
  </span>
);

const RequestPage: React.FC<
  IDockviewPanelProps<{
    node?: TreeCollectionNode;
    treeId: string;
    iconType: EntryKind;
    someRandomString: string;
  }>
> = (props) => {
  const { displayMode } = useRequestModeStore();

  const isDebug = React.useContext(DebugContext);

  let showEndpoint = false;
  let dontShowTabs = true;
  const [activeTab, setActiveTab] = React.useState(showEndpoint ? "endpoint" : "request");
  const [activeRequestTabId, setActiveRequestTabId] = React.useState("params");

  // Use RequestPage store
  const { requestData, httpMethod, setHttpMethod, updateRequestData } = useRequestPageStore();

  // Parse current URL from store

  if (props.params?.node) {
    showEndpoint = displayMode === "DESIGN_FIRST" && props.params.node.class === "Endpoint";
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
  };

  const handleUrlChange = React.useCallback(
    (url: string) => {
      // Prevent unnecessary updates if URL hasn't changed
      if (url === requestData.url.raw) {
        return;
      }

      const parsed = parseUrl(url);

      // Update store with complete data in one call to avoid multiple re-renders
      const updatedData = {
        url: {
          raw: url,
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

  // Define the request configuration tabs
  const requestTabs: TabItem[] = [
    {
      id: "params",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="SquareBrackets" className="h-4 w-4" />
          <span>Params</span>
          <Badge count={6} />
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
        icon={<Icon icon="Placeholder" className="size-[18px]" />}
        tabs={dontShowTabs ? null : tabs}
        toolbar={toolbar}
        props={props}
      />
      <PageContent className={cn("relative", isDebug && "border-2 border-dashed border-orange-500")}>
        <Breadcrumbs panelId={props.api.id} />

        <div className="flex h-full flex-col p-2">
          {props.params?.node ? (
            <div className="flex-1">
              {/* Request Input Section */}
              <div className="mb-6">
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
                  className="mb-4"
                />

                {/* Request Configuration Tabs */}
                {activeTab === "request" && (
                  <PageContainerWithTabs
                    tabs={requestTabs}
                    activeTabId={activeRequestTabId}
                    onTabChange={setActiveRequestTabId}
                    className="ml-0"
                    noPadding
                  />
                )}
              </div>
            </div>
          ) : (
            <div className="flex flex-1 items-center justify-center">
              <div className="text-center">
                <p className="mb-4 text-sm text-(--moss-secondary-text)">No request selected</p>
                {props?.params.someRandomString && (
                  <p className="text-xs text-(--moss-secondary-text)">Backend ID: {props.params.someRandomString}</p>
                )}
              </div>
            </div>
          )}

          {isDebug && (
            <Metadata
              onClick={() => {
                props.api.setRenderer(props.api.renderer === "always" ? "onlyWhenVisible" : "always");
              }}
              api={props.api}
            />
          )}
        </div>
      </PageContent>
    </PageView>
  );
};

export { RequestPage };
