// Generated by ts-to-zod
import { z } from "zod";

import { jsonValueSchema } from "@repo/bindings-utils";
import {
  activitybarPositionSchema,
  editorGridOrientationSchema,
  panelRendererSchema,
  sidebarPositionSchema,
} from "./primitives.zod";
import { type EditorGridNode } from "./types";

export const activitybarItemStateInfoSchema = z.object({
  id: z.string(),
  order: z.number(),
  visible: z.boolean(),
});

export const collectionInfoSchema = z.object({
  id: z.string(),
  displayName: z.string(),
  order: z.number().optional(),
});

export const editorGridLeafDataSchema = z.object({
  views: z.array(z.string()),
  activeView: z.string(),
  id: z.string(),
});

export const editorGridNodeSchema: z.ZodSchema<EditorGridNode> = z.lazy(() =>
  z.union([
    z.object({
      "type": z.literal("branch"),
      data: z.array(editorGridNodeSchema),
      size: z.number(),
    }),
    z.object({
      "type": z.literal("leaf"),
      data: editorGridLeafDataSchema,
      size: z.number(),
    }),
  ])
);

export const environmentInfoSchema = z.object({
  id: z.string(),
  collectionId: z.string().optional(),
  name: z.string(),
  order: z.number().optional(),
});

export const panelPartStateInfoSchema = z.object({
  size: z.number(),
  visible: z.boolean(),
});

export const workspaceModeSchema = z.union([z.literal("DESIGN_FIRST"), z.literal("REQUEST_FIRST")]);
export const activitybarPartStateInfoSchema = z.object({
  lastActiveContainerId: z.string().optional(),
  position: activitybarPositionSchema,
  items: z.array(activitybarItemStateInfoSchema),
});

export const editorGridStateSchema = z.object({
  root: editorGridNodeSchema,
  width: z.number(),
  height: z.number(),
  orientation: editorGridOrientationSchema,
});

export const editorPanelStateSchema = z.object({
  id: z.string(),
  contentComponent: z.string().optional(),
  tabComponent: z.string().optional(),
  title: z.string().optional(),
  renderer: panelRendererSchema.optional(),
  params: z.record(jsonValueSchema),
  minimumWidth: z.number().optional(),
  minimumHeight: z.number().optional(),
  maximumWidth: z.number().optional(),
  maximumHeight: z.number().optional(),
});

export const editorPartStateInfoSchema = z.object({
  grid: editorGridStateSchema,
  panels: z.record(editorPanelStateSchema),
  activeGroup: z.string().optional(),
});

export const sidebarPartStateInfoSchema = z.object({
  position: sidebarPositionSchema,
  size: z.number(),
  visible: z.boolean(),
});
