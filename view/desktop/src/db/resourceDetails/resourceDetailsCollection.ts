import { createCollection, localOnlyCollectionOptions } from "@tanstack/react-db";

import { resourceDetailSchema } from "./schemas/resourceDetailSchema";

export const resourceDetailsCollection = createCollection(
  localOnlyCollectionOptions({
    id: "resourceDetails",
    getKey: (item) => item.id,
    schema: resourceDetailSchema,
  })
);
