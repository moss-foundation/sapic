import { useState } from "react";

import { useDescribeWorkspaceState } from "@/hooks";

export const WorkspaceProvider = ({ children }: { children: React.ReactNode }) => {
  const [isInitialWorkspaceFetch, setIsInitialWorkspaceFetch] = useState(true);

  const { isLoading: isLoadingWorkspace } = useDescribeWorkspaceState({
    enabled: isInitialWorkspaceFetch,
  });

  if (isLoadingWorkspace) {
    return null;
  }

  return <>{children}</>;
};
