import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { ProfileInfo } from "@repo/moss-user";

import { ProfilePageProps } from "../../../ProfilePage";

interface OverviewTabProps extends IDockviewPanelProps<ProfilePageProps> {
  profile: ProfileInfo;
}

export const OverviewTab = ({ profile }: OverviewTabProps) => {
  return (
    <div className="flex flex-col gap-6">
      <section>
        <h3 className="mb-3 text-lg font-medium">Profile Information</h3>
        <div className="flex flex-col gap-4 rounded-md border border-(--moss-border-color) p-4">
          <div className="flex flex-col gap-1">
            <span className="text-sm text-(--moss-secondary-text)">Profile Name</span>
            <span className="text-base">{profile.name}</span>
          </div>
          <div className="flex flex-col gap-1">
            <span className="text-sm text-(--moss-secondary-text)">Profile ID</span>
            <span className="font-mono text-sm">{profile.id}</span>
          </div>
          <div className="flex flex-col gap-1">
            <span className="text-sm text-(--moss-secondary-text)">Connected Accounts</span>
            <span className="text-base">{profile.accounts.length}</span>
          </div>
        </div>
      </section>

      <section>
        <h3 className="mb-3 text-lg font-medium">Quick Stats</h3>
        <div className="grid grid-cols-3 gap-4">
          <div className="rounded-md border border-(--moss-border-color) p-4">
            <div className="text-sm text-(--moss-secondary-text)">Total Accounts</div>
            <div className="mt-1 text-2xl font-semibold">{profile.accounts.length}</div>
          </div>
          <div className="rounded-md border border-(--moss-border-color) p-4">
            <div className="text-sm text-(--moss-secondary-text)">GitHub Accounts</div>
            <div className="mt-1 text-2xl font-semibold">
              {profile.accounts.filter((a) => a.kind === "GITHUB").length}
            </div>
          </div>
          <div className="rounded-md border border-(--moss-border-color) p-4">
            <div className="text-sm text-(--moss-secondary-text)">GitLab Accounts</div>
            <div className="mt-1 text-2xl font-semibold">
              {profile.accounts.filter((a) => a.kind === "GITLAB").length}
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};
