import { useState } from "react";

import { FramedTabs } from "@/lib/ui";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";

import { AlertsTab, InsightsTab, IssuesTab, MockTab, OverviewTab, RunTab } from "./tabs";

export const EndpointViewBody = () => {
  const [activeTab, setActiveTab] = useState("run");

  const tabs = [
    {
      id: "overview",
      label: "Overview",
      content: <OverviewTab />,
    },
    {
      id: "run",
      label: "Run",
      content: <RunTab />,
    },
    {
      id: "issues",
      label: "Issues",
      content: <IssuesTab />,
    },
    {
      id: "alerts",
      label: "Alerts",
      content: <AlertsTab />,
    },
    {
      id: "insights",
      label: "Insights",
      content: <InsightsTab />,
    },
    {
      id: "mock",
      label: "Mock",
      content: <MockTab />,
    },
  ];

  return (
    <FramedTabs.Root value={activeTab} onValueChange={setActiveTab} className="flex flex-1 flex-col">
      <FramedTabs.List className="flex items-center justify-between">
        <div className="flex items-center">
          {tabs.map((tab) => (
            <FramedTabs.Trigger key={tab.id} value={tab.id}>
              {tab.label}
            </FramedTabs.Trigger>
          ))}
        </div>
        <ToolbarPlaceholder />
      </FramedTabs.List>

      {tabs.map((tab) => (
        <FramedTabs.Content key={tab.id} value={tab.id} className="flex flex-1">
          <PageWrapper className="flex flex-1 flex-col">{tab.content}</PageWrapper>
        </FramedTabs.Content>
      ))}
    </FramedTabs.Root>
  );
};

const ToolbarPlaceholder = () => {
  return (
    <div className="flex items-center gap-1.5 opacity-50">
      <div className="flex items-center gap-1">
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="7" cy="7" r="6.125" fill="#E55765" />
          <path
            d="M7.875 4.375C7.875 3.89175 7.48325 3.5 7 3.5C6.51675 3.5 6.125 3.89175 6.125 4.375V6.5625C6.125 7.04575 6.51675 7.4375 7 7.4375C7.48325 7.4375 7.875 7.04575 7.875 6.5625L7.875 4.375Z"
            fill="white"
          />
          <path
            d="M7 10.5C7.48325 10.5 7.875 10.1082 7.875 9.625C7.875 9.14175 7.48325 8.75 7 8.75C6.51675 8.75 6.125 9.14175 6.125 9.625C6.125 10.1082 6.51675 10.5 7 10.5Z"
            fill="white"
          />
        </svg>

        <span>1</span>
      </div>
      <div className="flex items-center gap-1">
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            fillRule="evenodd"
            clipRule="evenodd"
            d="M0.276058 10.1047L5.3033 1.91295C6.05033 0.695683 7.94967 0.695685 8.6967 1.91295L13.7239 10.1047C14.5173 11.3974 13.506 13 11.8967 13H2.10327C0.494048 13 -0.517311 11.3974 0.276058 10.1047Z"
            fill="#FFAF0F"
          />
          <path
            d="M7.875 4.375C7.875 3.89175 7.48325 3.5 7 3.5C6.51675 3.5 6.125 3.89175 6.125 4.375V6.5625C6.125 7.04575 6.51675 7.4375 7 7.4375C7.48325 7.4375 7.875 7.04575 7.875 6.5625L7.875 4.375Z"
            fill="white"
          />
          <path
            d="M7 10.5C7.48325 10.5 7.875 10.1082 7.875 9.625C7.875 9.14175 7.48325 8.75 7 8.75C6.51675 8.75 6.125 9.14175 6.125 9.625C6.125 10.1082 6.51675 10.5 7 10.5Z"
            fill="white"
          />
        </svg>

        <span>2</span>
      </div>
    </div>
  );
};
