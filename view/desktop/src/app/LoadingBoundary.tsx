import { useState } from "react";

import { PageLoader } from "@/components";
import {
  USE_DESCRIBE_APP_QUERY_KEY,
  USE_DESCRIBE_COLOR_THEME_QUERY_KEY,
  USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY,
} from "@/hooks";
import { useIsFetching } from "@tanstack/react-query";

export const LoadingBoundary = ({ children }: { children: React.ReactNode }) => {
  const isFetchingApp = useIsFetching({ queryKey: [USE_DESCRIBE_APP_QUERY_KEY] });
  const isFetchingTheme = useIsFetching({ queryKey: [USE_DESCRIBE_COLOR_THEME_QUERY_KEY] });
  const isFetchingWorkspace = useIsFetching({ queryKey: [USE_DESCRIBE_WORKSPACE_STATE_QUERY_KEY] });

  const [isFirstWorkspaceFetch, setIsFirstWorkspaceFetch] = useState(true);

  const isLoading = isFetchingApp || isFetchingTheme || (isFetchingWorkspace && isFirstWorkspaceFetch);

  if (isLoading) {
    return <PageLoader />;
  }

  if (isFirstWorkspaceFetch) {
    setIsFirstWorkspaceFetch(false);
  }

  return <>{children}</>;
};
