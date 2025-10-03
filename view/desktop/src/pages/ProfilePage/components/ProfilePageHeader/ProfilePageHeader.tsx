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
      <header className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Icon icon="Person" className="size-6" />
            <h1 className="text-2xl font-semibold">{profile.name}</h1>
          </div>
        </div>
      </header>
    </PageWrapper>
  );
};
