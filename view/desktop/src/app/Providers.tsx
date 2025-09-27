import { ReactNode } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";
import { ActivityRouterProvider } from "./ActivityRouterProvider";
import { NotificationProvider } from "./NotificationProvider";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Providers = ({ children }: { children: ReactNode }) => {
  return (
    <ErrorBoundary>
      <ActivityRouterProvider>
        <NotificationProvider>
          <LanguageProvider>
            <ThemeProvider>{children}</ThemeProvider>
          </LanguageProvider>
        </NotificationProvider>
      </ActivityRouterProvider>
    </ErrorBoundary>
  );
};

export default Providers;
