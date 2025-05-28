import { useEffect } from "react";
import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { WorkspaceProvider } from "@/context/WorkspaceContext";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { Workspace } from "@/components/Workspace";
import { useDescribeAppState, useWorkspaceMapping } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const { data: appState, isLoading: isLoadingAppState } = useDescribeAppState();
  const { getNameById, workspaces } = useWorkspaceMapping();
  const { api } = useTabbedPaneStore();

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

  // Close welcome page when workspace is detected
  useEffect(() => {
    if (hasWorkspace) {
      const WelcomePanel = api?.getPanel("WelcomePage");
      if (WelcomePanel) {
        WelcomePanel.api.close();
      }
    }
  }, [hasWorkspace, api]);

  if (isLoadingAppState) {
    return <div>Loading workbench state...</div>;
  }

  return (
    <ActivityEventsProvider>
      <RootLayout>
        <AppLayout>
          {hasWorkspace ? (
            <WorkspaceProvider initialWorkspace={currentWorkspaceName}>
              <Workspace />
            </WorkspaceProvider>
          ) : (
            <EmptyWorkspace />
          )}
        </AppLayout>
      </RootLayout>
    </ActivityEventsProvider>
  );
};
