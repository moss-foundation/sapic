import { eq } from "@tanstack/db";
import { useLiveQuery } from "@tanstack/react-db";

import { resourceSummariesCollection } from "../resourceSummariesCollection";

export const useGetResourcesSummariesByProjectId = (projectId: string) => {
  const { data: resourcesSummaries } = useLiveQuery((q) =>
    q
      .from({ collection: resourceSummariesCollection })
      .where(({ collection }) => eq(collection.projectId, projectId))
      .orderBy(({ collection }) => collection.order)
  );

  return resourcesSummaries;
};
