import { EmptyWorkspace } from "@/components/EmptyWorkspace";
import { Workspace } from "@/components/Workspace";
import { useActiveWorkspace } from "@/hooks";
import { AppLayout, RootLayout } from "@/workbench/layouts";

export const Workbench = () => {
  const { activeWorkspace, hasActiveWorkspace } = useActiveWorkspace();

  return (
    <RootLayout>
      <AppLayout>
        {hasActiveWorkspace ? <Workspace workspaceName={activeWorkspace?.name} /> : <EmptyWorkspace />}
      </AppLayout>
    </RootLayout>
  );
};
