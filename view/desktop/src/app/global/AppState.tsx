import { useSyncColorTheme } from "@/hooks/useSyncColorTheme";
import { useSyncLanguage } from "@/hooks/useSyncLanguage";
import { PageLoader } from "@/workbench/ui/components";

import ErrorBoundary from "../ErrorBoundary";

interface AppInitStateProps {
  children: React.ReactNode;
}

export const AppState = ({ children }: AppInitStateProps) => {
  const { isInit: isInitLanguage } = useSyncLanguage();
  const { isInit: isInitColorTheme } = useSyncColorTheme();
  // const { isInit: isInitLayout } = useInitLayout();

  const isInit = isInitLanguage && isInitColorTheme;

  if (!isInit) {
    return <PageLoader className="bg-sky-200" />;
  }

  return <ErrorBoundary>{children}</ErrorBoundary>;
};
