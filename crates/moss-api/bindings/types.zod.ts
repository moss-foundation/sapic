// Generated by ts-to-zod
import { z } from "zod";

export const optionsSchema = z.object({
  request_id: z.string().optional(),
  timeout: z.bigint().optional(),
});
