import { z } from "zod";
import { projectSummarySchema } from "./schemas/projectSummarySchema";

export type ProjectSummary = z.infer<typeof projectSummarySchema>;
