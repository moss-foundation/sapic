// Generated by ts-to-zod
import { z } from "zod";
import {
  afterUpdateDirEntryDescriptionSchema,
  afterUpdateItemEntryDescriptionSchema,
  dirConfigurationModelSchema,
  itemConfigurationModelSchema,
  updateDirEntryParamsSchema,
  updateItemEntryParamsSchema,
} from "./types.zod";

export const batchCreateEntryOutputSchema = z.object({
  ids: z.array(z.string()),
});

export const batchUpdateEntryOutputSchema = z.record(z.never());

export const createEntryOutputSchema = z.object({
  id: z.string(),
});

export const deleteEntryInputSchema = z.object({
  id: z.string(),
});

export const deleteEntryOutputSchema = z.object({
  id: z.string(),
});

export const streamEntriesInputSchema = z.union([
  z.literal("LOAD_ROOT"),
  z.object({
    "RELOAD_PATH": z.string(),
  }),
]);

export const streamEntriesOutputSchema = z.record(z.never());
export const createItemEntryInputSchema = z.object({
  path: z.string(),
  name: z.string(),
  order: z.number(),
  configuration: itemConfigurationModelSchema,
});

export const createDirEntryInputSchema = z.object({
  path: z.string(),
  name: z.string(),
  order: z.number(),
  configuration: dirConfigurationModelSchema,
});

export const batchUpdateEntryKindSchema = z.union([
  z.object({
    "ITEM": updateItemEntryParamsSchema,
  }),
  z.object({
    "DIR": updateDirEntryParamsSchema,
  }),
]);

export const batchUpdateEntryOutputKindSchema = z.union([
  z.object({
    "ITEM": afterUpdateItemEntryDescriptionSchema,
  }),
  z.object({
    "DIR": afterUpdateDirEntryDescriptionSchema,
  }),
]);

export const createEntryInputSchema = z.union([
  z.object({
    "item": createItemEntryInputSchema,
  }),
  z.object({
    "dir": createDirEntryInputSchema,
  }),
]);

export const updateEntryInputSchema = z.union([
  z.object({
    "ITEM": updateItemEntryParamsSchema,
  }),
  z.object({
    "DIR": updateDirEntryParamsSchema,
  }),
]);

export const updateEntryOutputSchema = z.union([
  z.object({
    "ITEM": afterUpdateItemEntryDescriptionSchema,
  }),
  z.object({
    "DIR": afterUpdateDirEntryDescriptionSchema,
  }),
]);

export const batchCreateEntryKindSchema = z.union([
  z.object({
    "ITEM": createItemEntryInputSchema,
  }),
  z.object({
    "DIR": createDirEntryInputSchema,
  }),
]);

export const batchUpdateEntryInputSchema = z.object({
  entries: z.array(batchUpdateEntryKindSchema),
});

export const batchCreateEntryInputSchema = z.object({
  entries: z.array(batchCreateEntryKindSchema),
});
