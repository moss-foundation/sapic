import { useInitLayout } from "@/hooks/useInitLayout";
import { AppLayout, RootLayout } from "@/workbench/layouts";

export const WorkspaceShell = () => {
  useInitLayout();

  return (
    <RootLayout>
      <AppLayout />
    </RootLayout>
  );
};

export default WorkspaceShell;
