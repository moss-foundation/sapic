import { useDescribeApp } from "@/hooks/app/useDescribeApp";

export const useActiveWorkspace = () => {
  const { data: appState, isFetching } = useDescribeApp();

  const activeWorkspace = appState?.workspace || undefined;
  const activeWorkspaceId = appState?.workspace?.id || undefined;
  const hasActiveWorkspace = !!appState?.workspace && !isFetching;

  return {
    activeWorkspace,
    activeWorkspaceId,
    hasActiveWorkspace,
  };
};
