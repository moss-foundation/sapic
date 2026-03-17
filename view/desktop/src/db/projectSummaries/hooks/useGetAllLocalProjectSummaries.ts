import { useLiveQuery } from "@tanstack/react-db";

import { projectSummariesCollection } from "../projectSummaries";

export const useGetAllLocalProjectSummaries = () => {
  const { data, isLoading, isError } = useLiveQuery((q) => q.from({ collection: projectSummariesCollection }));

  return {
    data,
    isLoading,
    isError,
  };
};
