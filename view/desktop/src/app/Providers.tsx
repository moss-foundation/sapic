import ErrorBoundary from "@/app/ErrorBoundary";
import { Workbench } from "@/components";

import { ActivityRouterProvider } from "./ActivityRouterProvider";
import { LoadingBoundary } from "./LoadingBoundary";
import { NotificationsProvider } from "./NotificationsProvider";

const Providers = () => {
  return (
    <ErrorBoundary>
      <LoadingBoundary>
        <ActivityRouterProvider>
          <NotificationsProvider>
            <Workbench />
          </NotificationsProvider>
        </ActivityRouterProvider>
      </LoadingBoundary>
    </ErrorBoundary>
  );
};

export default Providers;
