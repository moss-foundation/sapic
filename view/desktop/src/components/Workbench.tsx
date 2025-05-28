import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { Workspace } from "@/components/Workspace";
import { useDescribeAppState, useWorkspaceMapping } from "@/hooks";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const { data: appState, isLoading: isLoadingAppState } = useDescribeAppState();
  const { getNameById } = useWorkspaceMapping();

  // Convert workspace ID to workspace name for compatibility
  const currentWorkspaceId = appState?.lastWorkspace;
  const currentWorkspaceName = currentWorkspaceId ? getNameById(currentWorkspaceId) : null;
  const hasWorkspace = !!currentWorkspaceName;

  if (isLoadingAppState) {
    return <div>Loading workbench state...</div>;
  }

  return (
    <ActivityEventsProvider>
      <RootLayout>
        <AppLayout>{hasWorkspace ? <Workspace workspaceName={currentWorkspaceName} /> : <EmptyWorkspace />}</AppLayout>
      </RootLayout>
    </ActivityEventsProvider>
  );
};
