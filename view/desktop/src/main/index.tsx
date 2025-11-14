import { useListWorkspaces } from "@/hooks";
import { useGetLayout } from "@/workbench/adapters/tanstackQuery/layout";
import { NotificationsProvider } from "@/workbench/providers/NotificationsProvider";
import { Outlet, useParams } from "@tanstack/react-router";

import { AppState } from "../app/global/AppState";
import { ActivityRouterProvider } from "../workbench/providers/ActivityRouterProvider";

const MainIndex = () => {
  const { workspaceId } = useParams({ strict: false });

  const { data: layout } = useGetLayout({ workspaceId });
  const { data: workspaces } = useListWorkspaces();

  console.log({
    params: { workspaceId },
    tanstackData: { layout, workspaces },
  });

  return (
    <AppState>
      <ActivityRouterProvider>
        <NotificationsProvider>
          <Outlet />
        </NotificationsProvider>
      </ActivityRouterProvider>
    </AppState>
  );
};

export default MainIndex;
