import { useMemo } from "react";

import { sortObjectsByOrder } from "@/utils";
import { and, isNull, isUndefined, not, useLiveQuery } from "@tanstack/react-db";

import { environmentSummariesCollection } from "../environmentSummaries";

export const useGetAllProjectEnvironments = () => {
  const { data: projectEnvironments, isLoading } = useLiveQuery((q) =>
    q
      .from({ collection: environmentSummariesCollection })
      .where((env) => and(not(isUndefined(env.collection.projectId)), not(isNull(env.collection.projectId))))
  );

  const sortedProjectEnvironmentsByOrder = useMemo(() => {
    if (!projectEnvironments) return undefined;
    return sortObjectsByOrder(projectEnvironments);
  }, [projectEnvironments]);

  return { projectEnvironments, sortedProjectEnvironmentsByOrder, isLoading };
};
