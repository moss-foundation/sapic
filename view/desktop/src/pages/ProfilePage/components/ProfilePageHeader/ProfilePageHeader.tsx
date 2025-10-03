import { PageWrapper } from "@/components/PageView/PageWrapper";
import { DockviewPanelApi } from "@/lib/moss-tabs/src";
import { Icon } from "@/lib/ui";
import { ProfileInfo } from "@repo/moss-user";

interface ProfilePageHeaderProps {
  profile: ProfileInfo;
  api: DockviewPanelApi;
}

export const ProfilePageHeader = ({ profile }: ProfilePageHeaderProps) => {
  return (
    <PageWrapper>
      <header className="flex flex-col gap-2">
        <div className="flex items-center gap-2">
          <Icon icon="Person" className="size-5" />
          <h1 className="text-xl font-medium">{profile.name}</h1>
        </div>
      </header>
    </PageWrapper>
  );
};
