import { useMemo } from "react";

import { sortObjectsByOrder } from "@/utils";
import { isNull, isUndefined, or, useLiveQuery } from "@tanstack/react-db";

import { environmentSummariesCollection } from "../environmentSummaries";

export const useGetWorkspaceEnvironments = () => {
  const { data: workspaceEnvironments, isLoading } = useLiveQuery((q) =>
    q
      .from({ collection: environmentSummariesCollection })
      .where((env) => or(isUndefined(env.collection.projectId), isNull(env.collection.projectId)))
  );

  const sortedWorkspaceEnvironmentsByOrder = useMemo(() => {
    if (!workspaceEnvironments) return undefined;
    return sortObjectsByOrder(workspaceEnvironments);
  }, [workspaceEnvironments]);

  return {
    workspaceEnvironments,
    sortedWorkspaceEnvironmentsByOrder,
    isLoading,
  };
};
