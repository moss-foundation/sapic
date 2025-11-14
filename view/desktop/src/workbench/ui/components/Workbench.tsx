import { useActiveWorkspace } from "@/hooks";
import { AppLayout, RootLayout } from "@/workbench/layouts";
import { EmptyWorkspace } from "@/workbench/ui/components/EmptyWorkspace";
import { Workspace } from "@/workbench/ui/components/Workspace";

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
