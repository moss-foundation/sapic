import { Workbench } from "@/workbench";
import { Outlet } from "@tanstack/react-router";

import { AppState } from "../app/global/AppState";

const MainIndex = () => {
  return (
    <AppState>
      <Workbench>
        <Outlet />
      </Workbench>
    </AppState>
  );
};

export default MainIndex;
