import { useDescribeApp } from "@/hooks/app/useDescribeApp";

export const useProfileData = () => {
  const { data: appState, isLoading, error } = useDescribeApp();

  return {
    profile: appState?.profile || null,
    isLoading,
    error,
  };
};
