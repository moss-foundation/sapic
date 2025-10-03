import { useState } from "react";

import { PageWrapper } from "@/components/PageView/PageWrapper";
import { FramedTabs } from "@/lib/ui";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import { ProfileInfo } from "@repo/moss-user";

import { ProfilePageProps } from "../../ProfilePage";
import { AccountsTab, OverviewTab } from "./tabs";

interface ProfilePageBodyProps extends IDockviewPanelProps<ProfilePageProps> {
  profile: ProfileInfo;
}

export const ProfilePageBody = ({ profile, ...props }: ProfilePageBodyProps) => {
  const [activeTab, setActiveTab] = useState("overview");

  const tabs = [
    {
      id: "overview",
      label: "Overview",
      content: <OverviewTab profile={profile} {...props} />,
    },
    {
      id: "accounts",
      label: "Accounts",
      content: <AccountsTab profile={profile} {...props} />,
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
      </FramedTabs.List>

      {tabs.map((tab) => (
        <FramedTabs.Content key={tab.id} value={tab.id} className="flex flex-1">
          <PageWrapper className="flex flex-1 flex-col">{tab.content}</PageWrapper>
        </FramedTabs.Content>
      ))}
    </FramedTabs.Root>
  );
};
