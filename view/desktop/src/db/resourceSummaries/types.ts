import { z } from "zod";

import { resourceSummarySchema } from "./schemas/resourceSummarySchema";

export type LocalResourceSummary = z.infer<typeof resourceSummarySchema>;
