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
      />
      <PageContent className={cn("relative", isDebug && "border-2 border-dashed border-orange-500")}>
        <Breadcrumbs panelId={props.api.id} />

        <span className="pointer-events-none absolute top-1/2 left-1/2 flex -translate-x-1/2 -translate-y-1/2 transform flex-col text-[42px] opacity-50">
          {props.params?.node ? (
            <div>
              <span className="text-[18px]">Node name: "{props.params.node.name}"</span>
              <div className="pointer-events-auto max-h-[70vh] overflow-y-auto text-[12px]">
                <pre>{JSON.stringify(props.params.node, null, 2)}</pre>
              </div>
            </div>
          ) : (
            <>
              <span>{props.api.title}</span>
              <span>{Math.random().toFixed(2)}</span>
              {props?.params.someRandomString && (
                <span className="text-xs">some random string from backend: {props.params.someRandomString}</span>
              )}
            </>
          )}
        </span>

        {isDebug && (
          <Metadata
            onClick={() => {
              props.api.setRenderer(props.api.renderer === "always" ? "onlyWhenVisible" : "always");
            }}
            api={props.api}
          />
        )}
      </PageContent>
    </PageView>
  );
};

export default RequestPage;
