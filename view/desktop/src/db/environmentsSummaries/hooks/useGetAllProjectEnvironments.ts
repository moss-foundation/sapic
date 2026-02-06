import { useCurrentWorkspace } from "@/hooks";
import { and, isNull, isUndefined, not, useLiveQuery } from "@tanstack/react-db";

import { environmentSummariesCollection } from "../environmentSummaries";

export const useGetAllProjectEnvironments = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projectEnvironments, isLoading } = useLiveQuery(
    (q) => {
      return q
        .from({ collection: environmentSummariesCollection })
        .where((env) => and(not(isUndefined(env.collection.projectId)), not(isNull(env.collection.projectId))))
        .orderBy((env) => env.collection.order);
    },
    [currentWorkspaceId]
  );

  return { projectEnvironments, isLoading };
};
