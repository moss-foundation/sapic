import { PageView } from "@/components";
import { PageWrapper } from "@/components/PageView/PageWrapper";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { ProfilePageBody } from "./components/ProfilePageBody";
import { ProfilePageHeader } from "./components/ProfilePageHeader";
import { useProfileData } from "./hooks/useProfileData";

export interface ProfilePageProps {
  // Profile page doesn't need params from outside, it reads active profile
}

const ProfilePage = ({ ...props }: IDockviewPanelProps<ProfilePageProps>) => {
  const { profile, isLoading, error } = useProfileData();

  if (isLoading) {
    return (
      <PageWrapper>
        <div className="flex flex-1 items-center justify-center">
          <div className="text-center">
            <p className="mb-4 text-sm text-(--moss-secondary-text)">Loading profile...</p>
          </div>
        </div>
      </PageWrapper>
    );
  }

  if (error || !profile) {
    return (
      <PageWrapper>
        <div className="flex flex-1 items-center justify-center">
          <div className="text-center">
            <p className="mb-4 text-sm text-(--moss-secondary-text)">
              {error ? "Error loading profile" : "No profile found"}
            </p>
          </div>
        </div>
      </PageWrapper>
    );
  }

  return (
    <PageView>
      <ProfilePageHeader profile={profile} api={props.api} />
      <ProfilePageBody profile={profile} {...props} />
    </PageView>
  );
};

export { ProfilePage };
