import { IDockviewPanelProps } from "moss-tabs";

import { FramedTabs } from "@/lib/ui";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";
import { ProfileInfo } from "@repo/base";

import { ProfileViewProps } from "../../ProfileView";
import { OverviewTab } from "./tabs/OverviewTab";

interface ProfileViewBodyProps extends IDockviewPanelProps<ProfileViewProps> {
  profile: ProfileInfo;
  refetchProfile: () => void;
}

export const ProfileViewBody = ({ profile, refetchProfile, ...props }: ProfileViewBodyProps) => {
  return (
    <FramedTabs.Root value="overview" onValueChange={() => {}} className="flex flex-1 flex-col">
      <FramedTabs.List className="flex items-center justify-between">
        <div className="flex items-center">
          <FramedTabs.Trigger value="overview">Overview</FramedTabs.Trigger>
        </div>
      </FramedTabs.List>

      <FramedTabs.Content value="overview" className="flex flex-1">
        <PageWrapper className="pl-7.5 flex flex-1 flex-col">
          <OverviewTab profile={profile} refetchProfile={refetchProfile} {...props} />
        </PageWrapper>
      </FramedTabs.Content>
    </FramedTabs.Root>
  );
};
