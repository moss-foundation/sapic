import { useDescribeApp } from "@/hooks/app/useDescribeApp";

export const useActiveWorkspace = () => {
  const { data: appState, isLoading } = useDescribeApp();

  const activeWorkspace = appState?.workspace || null;
  const activeWorkspaceId = appState?.workspace?.id || null;
  const hasActiveWorkspace = !!appState?.workspace && !isLoading;

  return {
    activeWorkspace,
    activeWorkspaceId,
    hasActiveWorkspace,
  };
};
