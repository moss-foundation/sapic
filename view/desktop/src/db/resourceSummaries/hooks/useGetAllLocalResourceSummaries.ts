import { useLiveQuery } from "@tanstack/react-db";

import { resourceSummariesCollection } from "../resourceSummariesCollection";

export const useGetAllLocalResourceSummaries = () => {
  const { data, isLoading } = useLiveQuery((q) => q.from({ collection: resourceSummariesCollection }));

  return {
    data,
    isLoading,
  };
};
