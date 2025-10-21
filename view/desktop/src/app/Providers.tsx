import { ReactNode } from "react";
import { Toaster } from "sonner";

import ErrorBoundary from "@/components/ErrorBoundary";

import { ActivityRouterProvider } from "./ActivityRouterProvider";
import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Providers = ({ children }: { children: ReactNode }) => {
  return (
    <ErrorBoundary>
      <ActivityRouterProvider>
        <LanguageProvider>
          <ThemeProvider>
            {children}
            <Toaster
              position="bottom-right"
              richColors={false}
              toastOptions={{
                style: {
                  background: "var(--moss-notification-background)",
                  color: "var(--moss-notification-foreground)",
                  borderRadius: "8px",
                  width: "360px",
                },
              }}
            />
          </ThemeProvider>
        </LanguageProvider>
      </ActivityRouterProvider>
    </ErrorBoundary>
  );
};

export default Providers;
