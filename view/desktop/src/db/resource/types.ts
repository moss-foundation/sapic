import { z } from "zod";

import { resourceDetailsSchema } from "./schemas/resourceDetailsSchema";

export type ResourceDetails = z.infer<typeof resourceDetailsSchema>;
