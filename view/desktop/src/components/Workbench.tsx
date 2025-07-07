import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { PageLoader } from "@/components/PageLoader";
import { Workspace } from "@/components/Workspace";
import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
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
        <AppLayout>{hasWorkspace ? <Workspace workspaceName={workspace!.name} /> : <EmptyWorkspace />}</AppLayout>
      </RootLayout>
    </ActivityEventsProvider>
  );
};
