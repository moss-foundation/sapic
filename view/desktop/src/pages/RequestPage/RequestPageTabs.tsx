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
      <OutlinedTabs.List className="flex items-center justify-between">
        <div className="flex items-center">
          {tabs.map((tab) => (
            <OutlinedTabs.Trigger key={tab.id} value={tab.id}>
              {tab.label}
            </OutlinedTabs.Trigger>
          ))}
        </div>
        <ToolbarPlaceholder />
      </OutlinedTabs.List>

      {tabs.map((tab) => (
        <OutlinedTabs.Content key={tab.id} value={tab.id}>
          <PageWrapper>{tab.content}</PageWrapper>
        </OutlinedTabs.Content>
      ))}
    </OutlinedTabs.Root>
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
