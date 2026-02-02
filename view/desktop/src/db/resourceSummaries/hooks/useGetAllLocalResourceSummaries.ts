import { useLiveQuery } from "@tanstack/react-db";

import { resourceSummariesCollection } from "../resourceSummariesCollection";

export const useGetAllLocalResourceSummaries = () => {
  const { data: localResourceSummaries } = useLiveQuery((q) => q.from({ collection: resourceSummariesCollection }));

  return localResourceSummaries;
};
