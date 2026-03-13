import { environmentSummariesCollection } from "../environmentSummaries";

export const flushEnvironmentSummaries = () => {
  const ids = environmentSummariesCollection.map((p) => p.id);
  ids.forEach((id) => environmentSummariesCollection.delete(id));
};
