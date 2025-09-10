import { useState } from "react";

import { OutlinedTabs } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { RequestEntryTabs } from "./RequestEntryTabs/RequestEntryTabs";
import { RequestPageProps } from "./RequestPage";

export const RequestPageTabs = ({ ...props }: IDockviewPanelProps<RequestPageProps>) => {
  const [activeTab, setActiveTab] = useState("run");

  const tabs = [
    {
      id: "run",
      label: "Run",
      content: <RequestEntryTabs {...props} />,
    },
    {
      id: "issues",
      label: "Issues",
      content: <div>Issues</div>,
    },
    {
      id: "alerts",
      label: "Alerts",
      content: <div>Alerts</div>,
    },
    {
      id: "insights",
      label: "Insights",
      content: <div>Insights</div>,
    },
    {
      id: "mock",
      label: "Mock",
      content: <div>Mock</div>,
    },
  ];

  return (
    <OutlinedTabs.Root value={activeTab} onValueChange={setActiveTab}>
      <OutlinedTabs.List>
        {tabs.map((tab) => (
          <OutlinedTabs.Trigger key={tab.id} value={tab.id}>
            {tab.label}
          </OutlinedTabs.Trigger>
        ))}
      </OutlinedTabs.List>

      {tabs.map((tab) => (
        <OutlinedTabs.Content key={tab.id} value={tab.id}>
          <PageWrapper>{tab.content}</PageWrapper>
        </OutlinedTabs.Content>
      ))}
    </OutlinedTabs.Root>
  );
};
