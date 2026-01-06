import { z } from "zod";

import { resourceDetailSchema } from "./schemas/resourceDetailSchema";

export type ResourceDetails = z.infer<typeof resourceDetailSchema>;
