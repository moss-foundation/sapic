import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { Workspace } from "@/components/Workspace";
import { PageLoader } from "@/components/PageLoader";
import { useDescribeAppState, useWorkspaceMapping } from "@/hooks";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const { data: appState, isLoading: isLoadingAppState } = useDescribeAppState();
  const { getWorkspaceById } = useWorkspaceMapping();

  const activeWorkspaceId = appState?.lastWorkspace;
  const activeWorkspace = activeWorkspaceId ? getWorkspaceById(activeWorkspaceId) : null;
  const hasWorkspace = !!activeWorkspace;

  if (isLoadingAppState) {
    return <PageLoader />;
  }

  return (
    <ActivityEventsProvider>
      <RootLayout>
        <AppLayout>
          {hasWorkspace ? <Workspace workspaceName={activeWorkspace.displayName} /> : <EmptyWorkspace />}
        </AppLayout>
      </RootLayout>
    </ActivityEventsProvider>
  );
};
