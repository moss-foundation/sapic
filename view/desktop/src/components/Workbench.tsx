import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { PageLoader } from "@/components/PageLoader";
import { Workspace } from "@/components/Workspace";
import { useActiveWorkspace } from "@/hooks";
import { useDescribeApp } from "@/hooks/app/useDescribeApp";
import { AppLayout, RootLayout } from "@/layouts";

export const Workbench = () => {
  const { activeWorkspace, hasActiveWorkspace } = useActiveWorkspace();
  const { isLoading } = useDescribeApp();

  if (isLoading) {
    return <PageLoader />;
  }

  return (
    <RootLayout>
      <AppLayout>
        {hasActiveWorkspace ? <Workspace workspaceName={activeWorkspace?.name} /> : <EmptyWorkspace />}
      </AppLayout>
    </RootLayout>
  );
};
