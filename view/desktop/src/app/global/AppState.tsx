import { useSyncSettings } from "@/hooks/app/useSyncSettings";
import { PageLoader } from "@/workbench/ui/components";

import ErrorBoundary from "../ErrorBoundary";

interface AppInitStateProps {
  children: React.ReactNode;
}

export const AppState = ({ children }: AppInitStateProps) => {
  // TODO: Redo this to retrieve language and color theme settings
  // through a batch operation using SettingsStore.
  const { isSynced: areSettingsSynced } = useSyncSettings();

  if (!areSettingsSynced) {
    return <PageLoader className="bg-sky-200" />;
  }

  return <ErrorBoundary>{children}</ErrorBoundary>;
};
