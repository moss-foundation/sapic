import { createCollection, localOnlyCollectionOptions } from "@tanstack/react-db";

import { resourceSummarySchema } from "./schemas/resourceSummarySchema";

export const resourceSummariesCollection = createCollection(
  localOnlyCollectionOptions({
    id: "resourceSummaries",
    getKey: (item) => item.id,
    schema: resourceSummarySchema,
  })
);
