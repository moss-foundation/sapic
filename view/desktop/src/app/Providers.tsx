import { ReactNode, useEffect, useRef } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";
import { useDescribeAppState } from "@/hooks/appState/useDescribeAppState";
import { useOpenWorkspace } from "@/hooks/workbench/useOpenWorkspace";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";
import { applyColorThemeFromCache } from "@/utils/applyTheme";
import { useQueryClient } from "@tanstack/react-query";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Providers = ({ children }: { children: ReactNode }) => {
  useInitializeAppState();

  return (
    <ErrorBoundary>
      <LanguageProvider>
        <ThemeProvider>{children}</ThemeProvider>
      </LanguageProvider>
    </ErrorBoundary>
  );
};

const useInitializeAppState = () => {
  const queryClient = useQueryClient();

  const { data } = useDescribeAppState();
  const { mutate: openWorkspace } = useOpenWorkspace();
  const { activeWorkspace } = useActiveWorkspace();
  const hasTriedRestoration = useRef(false);

  // Initialize app theme and language
  useEffect(() => {
    if (data) {
      const theme = data.preferences?.theme ?? data.defaults.theme;

      document.querySelector("html")?.setAttribute("data-theme", theme.mode);

      applyColorThemeFromCache(theme.identifier, queryClient);
    }
  }, [data, queryClient]);

  // Restore previous workspace if available
  useEffect(() => {
    if (data && !hasTriedRestoration.current) {
      hasTriedRestoration.current = true;

      // Only restore if there's a previous workspace ID and no currently active workspace
      if (data.prevWorkspaceId && !activeWorkspace) {
        console.log("Restoring previous workspace:", data.prevWorkspaceId);
        openWorkspace(data.prevWorkspaceId, {
          onSuccess: () => {
            console.log("Successfully restored previous workspace");
          },
          onError: (error) => {
            console.error("Failed to restore previous workspace:", error.message);
          },
        });
      }
    }
  }, [data, activeWorkspace, openWorkspace]);
};

export default Providers;
