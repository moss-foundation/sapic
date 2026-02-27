import { useLiveQuery } from "@tanstack/react-db";

import { projectSummariesCollection } from "../projectSummaries";

export const useGetAllLocalProjectSummaries = () => {
  return useLiveQuery((q) => q.from({ collection: projectSummariesCollection }));
};
