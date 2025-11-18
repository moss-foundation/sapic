import { useDescribeApp } from "@/hooks/app/useDescribeApp";

/**
 * @deprecated we now use useParams to get the current workspace on window. Technically we can't be out of workspace if we are in main window
 */
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
