import { ReactNode } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";
import { ActivityRouterProvider } from "./ActivityRouterProvider";
import { NotificationProvider } from "./NotificationProvider";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Providers = ({ children }: { children: ReactNode }) => {
  return (
    <ErrorBoundary>
      <NotificationProvider>
        <ActivityRouterProvider>
          <LanguageProvider>
            <ThemeProvider>{children}</ThemeProvider>
          </LanguageProvider>
        </ActivityRouterProvider>
      </NotificationProvider>
    </ErrorBoundary>
  );
};

export default Providers;
