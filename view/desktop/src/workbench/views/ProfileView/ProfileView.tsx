import { IDockviewPanelProps } from "moss-tabs";

import { useDescribeApp } from "@/hooks";
import { PageView } from "@/workbench/ui/components";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";

import { ProfileViewBody } from "./components/ProfileViewBody";
import { ProfileViewHeader } from "./components/ProfileViewHeader";

export type ProfileViewProps = Record<string, never>;

const ProfileView = ({ ...props }: IDockviewPanelProps<ProfileViewProps>) => {
  const { data: appState, isLoading, error, refetch } = useDescribeApp();
  const profile = appState?.profile;

  if (error) {
    console.error("ProfileView error:", error);
  }

  if (isLoading) {
    return (
      <PageView>
        <PageWrapper>
          <div className="flex flex-1 items-center justify-center">
            <div className="text-center">
              <p className="text-(--moss-secondary-foreground) mb-4 text-sm">Loading profile...</p>
            </div>
          </div>
        </PageWrapper>
      </PageView>
    );
  }

  if (!profile) {
    return (
      <PageView>
        <PageWrapper>
          <div className="flex flex-1 items-center justify-center">
            <div className="text-center">
              <p className="text-(--moss-secondary-foreground) mb-4 text-sm">
                {error ? "Error loading profile" : "No profile found"}
              </p>
            </div>
          </div>
        </PageWrapper>
      </PageView>
    );
  }

  return (
    <PageView>
      <ProfileViewHeader profile={profile} api={props.api} />
      <ProfileViewBody profile={profile} refetchProfile={refetch} {...props} />
    </PageView>
  );
};

export { ProfileView };
