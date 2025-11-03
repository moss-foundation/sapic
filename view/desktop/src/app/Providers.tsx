import { ReactNode } from "react";

import ErrorBoundary from "@/app/ErrorBoundary";

import { ActivityRouterProvider } from "./ActivityRouterProvider";
import { LoadingBoundary } from "./LoadingBoundary";
import { NotificationsProvider } from "./NotificationsProvider";

const Providers = ({ children }: { children: ReactNode }) => {
  return (
    <ErrorBoundary>
      <LoadingBoundary>
        <ActivityRouterProvider>
          <NotificationsProvider>{children}</NotificationsProvider>
        </ActivityRouterProvider>
      </LoadingBoundary>
    </ErrorBoundary>
  );
};

export default Providers;
