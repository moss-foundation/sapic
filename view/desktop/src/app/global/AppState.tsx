import { useSyncColorTheme } from "@/hooks/useSyncColorTheme";
import { useSyncLanguage } from "@/hooks/useSyncLanguage";
import { PageLoader } from "@/workbench/ui/components";

import ErrorBoundary from "../ErrorBoundary";

interface AppInitStateProps {
  children: React.ReactNode;
}

export const AppState = ({ children }: AppInitStateProps) => {
  // TODO: Redo this to retrieve language and color theme settings
  // through a batch operation using SettingsStore.
  const { isInit: isInitLanguage } = useSyncLanguage();
  const { isInit: isInitColorTheme } = useSyncColorTheme();

  const isInit = isInitLanguage && isInitColorTheme;

  if (!isInit) {
    return <PageLoader className="bg-sky-200" />;
  }

  return <ErrorBoundary>{children}</ErrorBoundary>;
};
