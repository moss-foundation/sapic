import { IDockviewPanelProps } from "moss-tabs";

import { PageView } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";

import { ProfilePageBody } from "./components/ProfilePageBody";
import { ProfilePageHeader } from "./components/ProfilePageHeader";
import { useProfileData } from "./hooks/useProfileData";

export type ProfilePageProps = Record<string, never>;

const ProfilePage = ({ ...props }: IDockviewPanelProps<ProfilePageProps>) => {
  const { profile, isLoading, error, refetch } = useProfileData();

  // Add error boundary protection
  if (error) {
    console.error("ProfilePage error:", error);
  }

  if (isLoading) {
    return (
      <PageView>
        <PageWrapper>
          <div className="flex flex-1 items-center justify-center">
            <div className="text-center">
              <p className="mb-4 text-sm text-(--moss-secondary-text)">Loading profile...</p>
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
              <p className="mb-4 text-sm text-(--moss-secondary-text)">
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
      <ProfilePageHeader profile={profile} api={props.api} />
      <ProfilePageBody profile={profile} refetchProfile={refetch} {...props} />
    </PageView>
  );
};

export { ProfilePage };
