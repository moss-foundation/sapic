import { projectSummariesCollection } from "../projectSummaries";

export const flushProjectSummaries = () => {
  const ids = projectSummariesCollection.map((p) => p.id);
  ids.forEach((id) => projectSummariesCollection.delete(id));
};
