import { AppState } from "@/app/global/AppState";
import { Outlet } from "@tanstack/react-router";

const WelcomeIndex = () => {
  return (
    <AppState>
      <Outlet />
    </AppState>
  );
};

export default WelcomeIndex;
