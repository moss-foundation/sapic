import { useState, useEffect } from "react";
import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { WorkspaceProvider } from "@/context/WorkspaceContext";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { Workspace } from "@/components/Workspace";
import { useDescribeWorkbenchState } from "@/hooks/workspaces/useDescribeWorkbenchState";
import { useDescribeAppState } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const { data: workbenchState, isLoading: isLoadingWorkbench } = useDescribeWorkbenchState();
  const { data: appState, isLoading: isLoadingAppState } = useDescribeAppState();
  const [hasWorkspace, setHasWorkspace] = useState<boolean>(false);
  const { api } = useTabbedPaneStore();

  useEffect(() => {
    if (workbenchState) {
      setHasWorkspace(!!appState?.lastWorkspace);
    }
  }, [workbenchState, appState?.lastWorkspace]);

  console.log("----------------->hasWorkspace", hasWorkspace);

  // Close welcome page when workspace is detected
  useEffect(() => {
    if (appState?.lastWorkspace) {
      const WelcomePanel = api?.getPanel("WelcomePage");
      if (WelcomePanel) {
        WelcomePanel.api.close();
      }
    }
  }, [appState?.lastWorkspace, api]);

  if (isLoadingWorkbench || isLoadingAppState) {
    return <div>Loading workbench state...</div>;
  }

  return (
    <ActivityEventsProvider>
      <RootLayout>
        <AppLayout>
          {hasWorkspace ? (
            <WorkspaceProvider>
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
