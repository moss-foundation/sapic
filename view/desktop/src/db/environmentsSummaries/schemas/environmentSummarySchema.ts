import { z } from "zod";

export const environmentSummarySchema = z.object({
  id: z.string(),
  projectId: z.string().optional().nullable(),
  isActive: z.boolean(),
  name: z.string(),
  color: z.string().optional().nullable(),
  totalVariables: z.number(),

  order: z.number().optional(),
  expanded: z.boolean().default(false),
});
