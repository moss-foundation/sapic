import { useCurrentWorkspace } from "@/hooks";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { environmentSummariesCollection } from "../environmentSummaries";

export const useGetProjectEnvironments = (projectId?: string | null) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projectEnvironments, isLoading } = useLiveQuery(
    (q) => {
      if (!projectId) return undefined;

      return q
        .from({ collection: environmentSummariesCollection })
        .where((env) => eq(env.collection.projectId, projectId))
        .orderBy((env) => env.collection.order);
    },
    [projectId, currentWorkspaceId]
  );

  return { projectEnvironments, isLoading };
};
