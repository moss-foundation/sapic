import { createCollection, localOnlyCollectionOptions } from "@tanstack/db";

import { projectSummarySchema } from "./schemas/projectSummarySchema";

export const projectSummariesCollection = createCollection(
  localOnlyCollectionOptions({
    id: "projectSummaries",
    getKey: (item) => item.id,
    schema: projectSummarySchema,
  })
);
