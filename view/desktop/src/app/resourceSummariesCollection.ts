import { z } from "zod";

import {
  bodyInfoSchema,
  headerInfoSchema,
  pathParamInfoSchema,
  queryParamInfoSchema,
  resourceClassSchema,
  resourceKindSchema,
  resourceProtocolSchema,
} from "@repo/moss-project";
import { createCollection, localOnlyCollectionOptions } from "@tanstack/react-db";

const resourceDetailsSchema = z.object({
  id: z.string(),
  name: z.string(),
  class: resourceClassSchema,
  kind: resourceKindSchema,
  protocol: resourceProtocolSchema.optional(),
  url: z.string().optional(),
  headers: z.array(headerInfoSchema),
  pathParams: z.array(pathParamInfoSchema),
  queryParams: z.array(queryParamInfoSchema),
  body: bodyInfoSchema.optional(),
});

export type ResourceDetails = z.infer<typeof resourceDetailsSchema>;

export const resourceDetailsCollection = createCollection(
  localOnlyCollectionOptions({
    id: "resourceDetails",
    getKey: (item) => item.id,
    schema: resourceDetailsSchema,
  })
);
