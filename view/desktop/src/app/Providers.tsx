import { ReactNode } from "react";

import ErrorBoundary from "@/app/ErrorBoundary";

import { ActivityRouterProvider } from "./ActivityRouterProvider";
import LanguageProvider from "./LanguageProvider";
import { LoadingBoundary } from "./LoadingBoundary";
import { NotificationsProvider } from "./NotificationsProvider";
import ThemeProvider from "./ThemeProvider";

const Providers = ({ children }: { children: ReactNode }) => {
  return (
    <ErrorBoundary>
      <ActivityRouterProvider>
        <LanguageProvider>
          <ThemeProvider>
            <NotificationsProvider>
              <LoadingBoundary>{children}</LoadingBoundary>
            </NotificationsProvider>
          </ThemeProvider>
        </LanguageProvider>
      </ActivityRouterProvider>
    </ErrorBoundary>
  );
};

export default Providers;
