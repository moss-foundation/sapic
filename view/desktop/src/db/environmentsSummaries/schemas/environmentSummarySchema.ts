import { z } from "zod";

export const environmentSummarySchema = z.object({
  id: z.string(),
  projectId: z.string().optional().nullable(),
  isActive: z.boolean(),
  name: z.string(),
  color: z.string().optional().nullable(),
  totalVariables: z.number(),

  order: z.number(),
  expanded: z.boolean().optional(),

  metadata: z.object({
    isDirty: z.boolean().default(false),
  }),
});
