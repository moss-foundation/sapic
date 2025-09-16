import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { PageLoader } from "@/components/PageLoader";
import { Workspace } from "@/components/Workspace";
import { ActivityEventsProvider } from "@/context/ActivityEventsContext";
import { useActiveWorkspace } from "@/hooks";
import { useDescribeApp } from "@/hooks/useDescribeApp";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const { activeWorkspace, hasActiveWorkspace } = useActiveWorkspace();
  const { isLoading } = useDescribeApp();

  if (isLoading) {
    return <PageLoader />;
  }

  return (
    <ActivityEventsProvider>
      <RootLayout>
        <AppLayout>
          {hasActiveWorkspace ? <Workspace workspaceName={activeWorkspace?.name} /> : <EmptyWorkspace />}
        </AppLayout>
      </RootLayout>
    </ActivityEventsProvider>
  );
};
