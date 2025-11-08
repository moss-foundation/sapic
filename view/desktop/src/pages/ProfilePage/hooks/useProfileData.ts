import { useDescribeApp } from "@/hooks/app/useDescribeApp";

/**
 * @deprecated just use useDescribeApp and destruct the profile from the appState
 */
export const useProfileData = () => {
  const { data: appState, isLoading, error, refetch } = useDescribeApp();

  return {
    profile: appState?.profile || null,
    isLoading,
    error,
    refetch,
  };
};
