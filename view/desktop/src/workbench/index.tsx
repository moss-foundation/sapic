import { usePrefetchWorkspaceLayout } from "@/hooks/workbench";
import { ActivityRouterProvider } from "@/workbench/providers/ActivityRouterProvider";
import { NotificationsProvider } from "@/workbench/providers/NotificationsProvider";

export const Workbench = ({ children }: { children: React.ReactNode }) => {
  usePrefetchWorkspaceLayout();

  return (
    <ActivityRouterProvider>
      <NotificationsProvider>{children}</NotificationsProvider>
    </ActivityRouterProvider>
  );
};
