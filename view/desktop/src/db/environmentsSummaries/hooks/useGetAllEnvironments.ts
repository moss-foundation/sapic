import { useCurrentWorkspace } from "@/hooks";
import { useLiveQuery } from "@tanstack/react-db";

import { environmentSummariesCollection } from "../environmentSummaries";

export const useGetAllEnvironments = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: environments, isLoading } = useLiveQuery(
    (q) => {
      return q.from({ collection: environmentSummariesCollection }).orderBy((env) => env.collection.order);
    },
    [currentWorkspaceId]
  );

  return { environments, isLoading };
};
