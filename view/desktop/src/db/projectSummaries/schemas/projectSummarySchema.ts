import { z } from "zod";

import { branchInfoSchema } from "@repo/ipc";

export const projectSummarySchema = z.object({
  id: z.string(),
  name: z.string(),
  branch: branchInfoSchema.optional().nullable(),
  iconPath: z.string().optional().nullable(),
  archived: z.boolean(),

  order: z.number(),
  expanded: z.boolean(),
});
