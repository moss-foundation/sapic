import { useDescribeApp } from "@/hooks/app/useDescribeApp";

export const useProfileData = () => {
  const { data: appState, isLoading, error, refetch } = useDescribeApp();

  return {
    profile: appState?.profile || null,
    isLoading,
    error,
    refetch,
  };
};
