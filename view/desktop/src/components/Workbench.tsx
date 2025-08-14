import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { PageLoader } from "@/components/PageLoader";
import { Workspace } from "@/components/Workspace";
import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { useActiveWorkspace, useDescribeAppState } from "@/hooks";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const { activeWorkspace } = useActiveWorkspace();
  const { isLoading } = useDescribeAppState();
  const hasWorkspace = !!activeWorkspace;

  if (isLoading) {
    return <PageLoader />;
  }

  return (
    <ActivityEventsProvider>
      <RootLayout>
        <AppLayout>{hasWorkspace ? <Workspace workspaceName={activeWorkspace.name} /> : <EmptyWorkspace />}</AppLayout>
      </RootLayout>
    </ActivityEventsProvider>
  );
};
