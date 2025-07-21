import React from "react";

import { ActionButton, Breadcrumbs, PageContent, PageHeader, PageTabs, PageToolbar, PageView } from "@/components";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { Icon } from "@/lib/ui";
import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";
import { EntryKind } from "@repo/moss-collection";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import Metadata from "../parts/TabbedPane/DebugComponents/Metadata";

const DebugContext = React.createContext<boolean>(false);

interface RequestPageProps
  extends IDockviewPanelProps<{
    node?: TreeCollectionNode;
    treeId: string;
    iconType: EntryKind;
    someRandomString: string;
  }> {}

const RequestPage: React.FC<RequestPageProps> = (props) => {
  const { displayMode } = useRequestModeStore();

  const isDebug = React.useContext(DebugContext);

  let showEndpoint = false;
  let dontShowTabs = true;
  const [activeTab, setActiveTab] = React.useState(showEndpoint ? "endpoint" : "request");

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

        <div className="flex h-full flex-col p-4">
          {props.params?.node ? (
            <div className="flex-1">
              <div className="mb-4">
                <p className="text-sm text-(--moss-secondary-text)">
                  {props.params.node.class} • {props.params.node.kind}
                </p>
              </div>

              <div className="rounded border border-(--moss-border-color) bg-(--moss-secondary-background) p-4">
                <h3 className="mb-2 text-sm font-medium text-(--moss-primary-text)">Node Details</h3>
                <div className="max-h-[60vh] overflow-y-auto">
                  <pre className="text-xs whitespace-pre-wrap text-(--moss-secondary-text)">
                    {JSON.stringify(props.params.node, null, 2)}
                  </pre>
                </div>
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

export default RequestPage;
