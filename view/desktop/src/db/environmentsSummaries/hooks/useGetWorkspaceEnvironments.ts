import { useMemo } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { sortObjectsByOrder } from "@/utils";
import { isNull, isUndefined, or, useLiveQuery } from "@tanstack/react-db";

import { environmentSummariesCollection } from "../environmentSummaries";

export const useGetWorkspaceEnvironments = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { data: workspaceEnvironments, isLoading } = useLiveQuery(
    (q) => {
      return q
        .from({ collection: environmentSummariesCollection })
        .where((env) => or(isUndefined(env.collection.projectId), isNull(env.collection.projectId)))
        .orderBy((env) => env.collection.order);
    },
    [currentWorkspaceId]
  );

  return {
    workspaceEnvironments,
    isLoading,
  };
};
