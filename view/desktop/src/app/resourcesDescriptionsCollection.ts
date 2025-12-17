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

const resourceDescriptionSchema = z.object({
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

export type ResourceDescription = z.infer<typeof resourceDescriptionSchema>;

export const resourcesDescriptionsCollection = createCollection(
  localOnlyCollectionOptions({
    id: "resourcesDescriptions",
    getKey: (item) => item.id,
    schema: resourceDescriptionSchema,
  })
);
