import { z } from "zod";

import { environmentSummarySchema } from "./schemas/environmentSummarySchema";

export type EnvironmentSummary = z.infer<typeof environmentSummarySchema>;
