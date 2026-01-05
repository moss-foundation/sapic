import { z } from "zod";

import { streamResourcesEventSchema } from "@repo/moss-project";
import { createCollection, localOnlyCollectionOptions } from "@tanstack/react-db";

export const resourceSummariesSchema = z.object({
  id: z.string(),
  resource: streamResourcesEventSchema.optional(),
});

export const resourceSummariesCollection = createCollection(
  localOnlyCollectionOptions({
    id: "resourceSummaries",
    getKey: (item) => item.id,
    schema: resourceSummariesSchema,
  })
);
