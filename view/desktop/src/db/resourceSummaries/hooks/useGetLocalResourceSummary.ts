import { eq, useLiveQuery } from "@tanstack/react-db";

import { resourceSummariesCollection } from "../resourceSummariesCollection";

export const useGetLocalResourceSummary = (resourceId: string) => {
  const { data: localResourceSummary } = useLiveQuery((q) =>
    q
      .from({ collection: resourceSummariesCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  return localResourceSummary;
};
