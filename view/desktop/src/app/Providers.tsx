import { ReactNode, useEffect, useRef } from "react";

import ErrorBoundary from "@/components/ErrorBoundary";
import { useDescribeAppState } from "@/hooks/app/useDescribeAppState";
import { useOpenWorkspace } from "@/hooks/workbench/useOpenWorkspace";
import { useActiveWorkspace } from "@/hooks/workspace/derived/useActiveWorkspace";

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
  const { data } = useDescribeAppState();
  const { mutate: openWorkspace } = useOpenWorkspace();
  const { activeWorkspace } = useActiveWorkspace();
  const hasTriedRestoration = useRef(false);

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
