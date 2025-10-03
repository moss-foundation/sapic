import { PageWrapper } from "@/components/PageView/PageWrapper";
import { FramedTabs } from "@/lib/ui";
import { IDockviewPanelProps } from "@repo/moss-tabs";
import { ProfileInfo } from "@repo/moss-user";

import { ProfilePageProps } from "../../ProfilePage";
import { OverviewTab } from "./tabs/OverviewTab";

interface ProfilePageBodyProps extends IDockviewPanelProps<ProfilePageProps> {
  profile: ProfileInfo;
}

export const ProfilePageBody = ({ profile, ...props }: ProfilePageBodyProps) => {
  return (
    <FramedTabs.Root value="overview" onValueChange={() => {}} className="flex flex-1 flex-col">
      <FramedTabs.List className="flex items-center justify-between">
        <div className="flex items-center">
          <FramedTabs.Trigger value="overview">Overview</FramedTabs.Trigger>
        </div>
      </FramedTabs.List>

      <FramedTabs.Content value="overview" className="flex flex-1">
        <PageWrapper className="flex flex-1 flex-col pl-7.5">
          <OverviewTab profile={profile} {...props} />
        </PageWrapper>
      </FramedTabs.Content>
    </FramedTabs.Root>
  );
};
