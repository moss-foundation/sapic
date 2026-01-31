import { eq, useLiveQuery } from "@tanstack/react-db";

import { projectSummariesCollection } from "../projectSummaries";

export const useGetLocalProjectSummary = (projectId: string) => {
  const { data: localProjectSummary } = useLiveQuery((q) =>
    q
      .from({ collection: projectSummariesCollection })
      .where(({ collection }) => eq(collection.id, projectId))
      .findOne()
  );

  return localProjectSummary;
};
