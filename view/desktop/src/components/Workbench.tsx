import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { Workspace } from "@/components/Workspace";
import { PageLoader } from "@/components/PageLoader";
import { useActiveWorkspace, useDescribeAppState } from "@/hooks";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const workspace = useActiveWorkspace();
  const { isLoading } = useDescribeAppState();
  const hasWorkspace = !!workspace;

  if (isLoading) {
    return <PageLoader />;
  }

  return (
    <ActivityEventsProvider>
      <RootLayout>
        <AppLayout>
          {hasWorkspace ? <Workspace workspaceName={workspace!.displayName} /> : <EmptyWorkspace />}
        </AppLayout>
      </RootLayout>
    </ActivityEventsProvider>
  );
};
