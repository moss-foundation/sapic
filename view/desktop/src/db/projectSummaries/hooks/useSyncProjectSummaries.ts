import { useEffect } from "react";

import { useCurrentWorkspace } from "@/hooks";

import { refreshProjectSummaries } from "../actions/refreshProjectSummaries";

export const useSyncProjectSummaries = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  useEffect(() => {
    refreshProjectSummaries({ currentWorkspaceId });
  }, [currentWorkspaceId]);

  return { isLoading: false, isPending: false, refreshProjectSummaries };
};
