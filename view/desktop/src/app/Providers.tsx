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
                  background: "var(--moss-notification-bg)",
                  color: "var(--moss-notification-text)",
                  border: "1px solid var(--moss-notification-button-outline)",
                  borderRadius: "8px",
                  padding: "10px 16px",
                  gap: "8px",
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
