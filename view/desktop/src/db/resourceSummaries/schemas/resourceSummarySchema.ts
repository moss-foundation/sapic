import { z } from "zod";

import { listProjectResourceItemSchema } from "@repo/ipc";

export const resourceSummarySchema = z.object({
  projectId: z.string(),
  id: listProjectResourceItemSchema.shape.id,
  name: listProjectResourceItemSchema.shape.name,
  path: listProjectResourceItemSchema.shape.path,
  class: listProjectResourceItemSchema.shape.class,
  kind: listProjectResourceItemSchema.shape.kind,
  protocol: listProjectResourceItemSchema.shape.protocol,

  order: z.number().optional(),
  expanded: z.boolean().optional(),
});
