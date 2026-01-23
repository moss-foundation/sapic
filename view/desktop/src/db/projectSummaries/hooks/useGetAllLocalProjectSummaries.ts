import { useLiveQuery } from "@tanstack/react-db";

import { projectSummariesCollection } from "../projectSummaries";

export const useGetAllLocalProjectSummaries = () => {
  const { data: localProjectSummaries } = useLiveQuery((q) => q.from({ collection: projectSummariesCollection }));
  return localProjectSummaries;
};
