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

export const resourceDetailsSchema = z.object({
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
  metadata: z.object({
    isDirty: z.boolean(),
  }),
});
