import { createCollection, localOnlyCollectionOptions } from "@tanstack/react-db";

import { resourceDetailsSchema } from "./schemas/resourceDetailsSchema";

export const resourceDetailsCollection = createCollection(
  localOnlyCollectionOptions({
    id: "resourceDetails",
    getKey: (item) => item.id,
    schema: resourceDetailsSchema,
  })
);
