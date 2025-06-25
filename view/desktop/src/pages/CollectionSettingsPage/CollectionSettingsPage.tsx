import { useState } from "react";

import { PageContainerWithTabs, TabItem } from "@/components/PageContainer";
import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { Icon } from "@/lib/ui";

import { AuthTabContent } from "./tabs/AuthTabContent";
import { HeadersTabContent } from "./tabs/HeadersTabContent";
import { OverviewTabContent } from "./tabs/OverviewTabContent";
import { PostRequestTabContent } from "./tabs/PostRequestTabContent";
import { PreRequestTabContent } from "./tabs/PreRequestTabContent";
import { VariablesTabContent } from "./tabs/VariablesTabContent";

// Badge component for tab numbers
const Badge = ({ count }: { count: number }) => (
  <span className="background-(--moss-tab-badge-color) inline-flex h-3.5 w-3.5 min-w-[14px] items-center justify-center rounded-full text-xs leading-none font-medium text-white">
    <span className="relative top-[0.5px]">{count}</span>
  </span>
);

// Indicator dot for status
const StatusDot = ({ active }: { active: boolean }) =>
  active ? <div className="background-(--moss-auth-indicator-color) h-2 w-2 rounded-full" /> : null;

export interface CollectionSettingsParams {
  collectionId: string;
}

export const CollectionSettings = ({ ...props }: IDockviewPanelProps<CollectionSettingsParams>) => {
  const { collectionId } = props.params;

  const [activeTabId, setActiveTabId] = useState("overview");

  if (!collectionId) {
    return (
      <div className="flex h-full items-center justify-center text-(--moss-primary-text)">
        <div className="text-center">
          <h2 className="text-lg font-semibold">No Active Collection</h2>
          <p className="text-sm">Please select a collection to view its settings.</p>
        </div>
      </div>
    );
  }

  // Define the tabs for the PageContainer matching the design
  const tabs: TabItem[] = [
    {
      id: "overview",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="SquareBrackets" className="h-4 w-4" />
          <span>Overview</span>
        </div>
      ),
      content: <OverviewTabContent collectionId={collectionId} />,
    },
    {
      id: "auth",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Auth" className="h-4 w-4" />
          <span>Auth</span>
          <StatusDot active={true} />
        </div>
      ),
      content: <AuthTabContent />,
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
      content: <HeadersTabContent />,
    },
    {
      id: "variables",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="Braces" className="h-4 w-4" />
          <span>Variables</span>
          <Badge count={3} />
        </div>
      ),
      content: <VariablesTabContent />,
    },
    {
      id: "pre-request",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="PreRequest" className="h-4 w-4" />
          <span>Pre Request</span>
        </div>
      ),
      content: <PreRequestTabContent />,
    },
    {
      id: "post-request",
      label: (
        <div className="flex items-center gap-1">
          <Icon icon="PostRequest" className="h-4 w-4" />
          <span>Post Request</span>
        </div>
      ),
      content: <PostRequestTabContent />,
    },
  ];

  return <PageContainerWithTabs tabs={tabs} activeTabId={activeTabId} onTabChange={setActiveTabId} />;
};
