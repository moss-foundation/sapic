import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { Workspace } from "@/components/Workspace";
import { useDescribeAppState, useWorkspaceMapping } from "@/hooks";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const { data: appState, isLoading: isLoadingAppState } = useDescribeAppState();
  const { getNameById, workspaces } = useWorkspaceMapping();

  // Convert workspace ID to workspace name for compatibility
  const currentWorkspaceId = appState?.lastWorkspace;
  const currentWorkspaceName = currentWorkspaceId ? getNameById(currentWorkspaceId) : null;
  const hasWorkspace = !!currentWorkspaceName;

  console.log("=== Workbench state ===");
  console.log("Current workspace ID from appState:", currentWorkspaceId);
  console.log(
    "Available workspaces:",
    workspaces.map((w) => ({ id: w.id, displayName: w.displayName }))
  );
  console.log("Mapped workspace name:", currentWorkspaceName);
  console.log("Has workspace:", hasWorkspace);

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
