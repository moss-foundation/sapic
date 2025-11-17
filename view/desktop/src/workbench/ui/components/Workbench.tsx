import { useInitLayout } from "@/hooks/useInitLayout";
import { AppLayout, RootLayout } from "@/workbench/layouts";

export const Workbench = () => {
  useInitLayout();
  return (
    <RootLayout>
      <AppLayout />
    </RootLayout>
  );
};
