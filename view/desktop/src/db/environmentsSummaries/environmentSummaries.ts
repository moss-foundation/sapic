import { createCollection, localOnlyCollectionOptions } from "@tanstack/react-db";

import { environmentSummarySchema } from "./schemas/environmentSummarySchema";

export const environmentSummariesCollection = createCollection(
  localOnlyCollectionOptions({
    id: "environmentSummaries",
    getKey: (item) => item.id,
    schema: environmentSummarySchema,
  })
);
