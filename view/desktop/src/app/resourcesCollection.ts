import { z } from "zod";

import { streamResourcesEventSchema } from "@repo/moss-project";
import { createCollection, localOnlyCollectionOptions } from "@tanstack/react-db";

export const resourcesCollectionSchema = z.object({
  id: z.string(),
  resource: streamResourcesEventSchema.optional(),
});

export const resourcesCollection = createCollection(
  localOnlyCollectionOptions({
    id: "resources",
    getKey: (item) => item.id,
    schema: resourcesCollectionSchema,
  })
);
