import { AppState } from "@/app/global/AppState";
import { Outlet } from "@tanstack/react-router";

const OnboardingIndex = () => {
  return (
    <AppState>
      <Outlet />
    </AppState>
  );
};

export default OnboardingIndex;
