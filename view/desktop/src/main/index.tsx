import { NotificationsProvider } from "@/workbench/providers/NotificationsProvider";
import { Outlet } from "@tanstack/react-router";

import { AppState } from "../app/global/AppState";
import { ActivityRouterProvider } from "../workbench/providers/ActivityRouterProvider";

const MainIndex = () => {
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
