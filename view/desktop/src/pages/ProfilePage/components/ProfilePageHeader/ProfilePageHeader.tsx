import { DockviewPanelApi } from "moss-tabs";

import { PageWrapper } from "@/components/PageView/PageWrapper";
import { Icon } from "@/lib/ui";
import { ProfileInfo } from "@repo/moss-user";

interface ProfilePageHeaderProps {
  profile: ProfileInfo;
  api: DockviewPanelApi;
}

export const ProfilePageHeader = ({ profile }: ProfilePageHeaderProps) => {
  return (
    <PageWrapper>
      <header className="flex flex-col gap-2 pb-2">
        <div className="flex items-center gap-2">
          <Icon icon="Profile" className="size-6.5" />
          <h1 className="text-lg font-medium">{profile.name}</h1>
        </div>
      </header>
    </PageWrapper>
  );
};
