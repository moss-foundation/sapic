import { useMemo } from "react";

import { sortObjectsByOrder } from "@/utils";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { environmentSummariesCollection } from "../environmentSummaries";

export const useGetProjectEnvironments = (projectId?: string | null) => {
  const { data: projectEnvironments, isLoading } = useLiveQuery(
    (q) => {
      if (!projectId) return undefined;

      return q
        .from({ collection: environmentSummariesCollection })
        .where((env) => eq(env.collection.projectId, projectId));
    },
    [projectId]
  );

  const sortedProjectEnvironmentsByOrder = useMemo(() => {
    if (!projectEnvironments) return undefined;
    return sortObjectsByOrder(projectEnvironments);
  }, [projectEnvironments]);

  return { projectEnvironments, sortedProjectEnvironmentsByOrder, isLoading };
};
