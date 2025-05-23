// Generated by ts-to-zod
import { localeInfoSchema } from "@repo/moss-nls";
import { colorThemeInfoSchema } from "@repo/moss-theme";
import { z } from "zod";
import { defaultsSchema, preferencesSchema } from "./types.zod";
export const describeAppStateOutputSchema = z.object({
  preferences: preferencesSchema,
  defaults: defaultsSchema,
  lastWorkspace: z.string().optional(),
});

export const setColorThemeInputSchema = z.object({
  themeInfo: colorThemeInfoSchema,
});

export const setLocaleInputSchema = z.object({
  localeInfo: localeInfoSchema,
});
