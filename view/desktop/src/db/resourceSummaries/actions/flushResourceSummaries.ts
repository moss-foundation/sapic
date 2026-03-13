import { resourceSummariesCollection } from "../resourceSummariesCollection";

export const flushResourceSummaries = () => {
  const ids = resourceSummariesCollection.map((p) => p.id);
  ids.forEach((id) => resourceSummariesCollection.delete(id));
};
