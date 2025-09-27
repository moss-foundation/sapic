import { ReactNode } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";
import { ActivityRouterProvider } from "./ActivityRouterProvider";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Providers = ({ children }: { children: ReactNode }) => {
  return (
    <ErrorBoundary>
      <ActivityRouterProvider>
        <LanguageProvider>
          <ThemeProvider>{children}</ThemeProvider>
        </LanguageProvider>
      </ActivityRouterProvider>
    </ErrorBoundary>
  );
};

export default Providers;
