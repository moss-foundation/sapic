import { createCollection, localOnlyCollectionOptions } from "@tanstack/db";

import { environmentSummarySchema } from "./schemas/environmentSummarySchema";

export const environmentSummariesCollection = createCollection(
  localOnlyCollectionOptions({
    id: "environmentSummaries",
    getKey: (item) => item.id,
    schema: environmentSummarySchema,
  })
);
