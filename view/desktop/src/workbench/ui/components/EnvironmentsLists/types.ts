import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ENVIRONMENT_ITEM_DRAG_TYPE, ENVIRONMENT_LIST_DRAG_TYPE } from "./constants";

export enum EnvironmentListType {
  GLOBAL = ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL,
  GROUPED = ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED,
  GROUPED_LIST = ENVIRONMENT_LIST_DRAG_TYPE.GROUPED,
}

export interface DragEnvironmentItem {
  type: EnvironmentListType;
  data: {
    environment: EnvironmentSummary;
  };
  [key: string | symbol]: unknown;
}

export interface DropEnvironmentItem {
  type: EnvironmentListType;
  data: {
    environment: EnvironmentSummary;
  };
  instruction?: Instruction;
  [key: string | symbol]: unknown;
}

export type DropOperation = "ReorderGlobals" | "ReorderGrouped" | "MoveToGlobal" | "MoveToGrouped" | "CombineToGrouped";
