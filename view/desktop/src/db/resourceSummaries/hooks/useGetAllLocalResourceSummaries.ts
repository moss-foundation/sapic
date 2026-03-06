import { useLiveQuery } from "@tanstack/react-db";

import { resourceSummariesCollection } from "../resourceSummariesCollection";

export const useGetAllLocalResourceSummaries = () => {
  return useLiveQuery((q) => q.from({ collection: resourceSummariesCollection }));
};
