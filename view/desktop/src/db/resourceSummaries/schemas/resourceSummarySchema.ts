import { z } from "zod";

import { streamResourcesEventSchema } from "@repo/moss-project";

export const resourceSummarySchema = z.object({
  projectId: z.string(),
  id: streamResourcesEventSchema.shape.id,
  name: streamResourcesEventSchema.shape.name,
  path: streamResourcesEventSchema.shape.path,
  class: streamResourcesEventSchema.shape.class,
  kind: streamResourcesEventSchema.shape.kind,
  protocol: streamResourcesEventSchema.shape.protocol,
  order: streamResourcesEventSchema.shape.order,
  expanded: streamResourcesEventSchema.shape.expanded,
});
