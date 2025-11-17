import { NotificationsProvider } from "@/workbench/providers/NotificationsProvider";
import { Outlet } from "@tanstack/react-router";

import { AppState } from "../app/global/AppState";
import { ActivityRouterProvider } from "../workbench/providers/ActivityRouterProvider";
import { MainState } from "./MainState";

const MainIndex = () => {
  return (
    <AppState>
      <MainState>
        <ActivityRouterProvider>
          <NotificationsProvider>
            <Outlet />
          </NotificationsProvider>
        </ActivityRouterProvider>
      </MainState>
    </AppState>
  );
};

export default MainIndex;
