import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { EnvironmentGroup, StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { ENVIRONMENT_ITEM_DRAG_TYPE, ENVIRONMENT_LIST_DRAG_TYPE } from "./constants";

export interface GroupedEnvironments extends EnvironmentGroup {
  environments: StreamEnvironmentsEvent[];
}

export interface GlobalEnvironmentItem {
  type: ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL;
  data: {
    environment: StreamEnvironmentsEvent;
  };
  instruction?: Instruction;
}

export interface GroupedEnvironmentItem {
  type: ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED;
  data: {
    environment: StreamEnvironmentsEvent;
  };
  instruction?: Instruction;
}

export interface GroupedEnvironmentList {
  type: ENVIRONMENT_LIST_DRAG_TYPE.GROUPED;
  data: {
    groupWithEnvironments: GroupedEnvironments;
  };
  instruction?: Instruction;
  [key: string | symbol]: unknown;
}

export type EnvironmentListType =
  | ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL
  | ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED
  | ENVIRONMENT_LIST_DRAG_TYPE.GROUPED;

export interface DragEnvironmentItem {
  type: EnvironmentListType;
  data: {
    environment: StreamEnvironmentsEvent;
  };
  [key: string | symbol]: unknown;
}

export interface DropEnvironmentItem {
  type: EnvironmentListType;
  data: {
    environment: StreamEnvironmentsEvent;
  };
  instruction?: Instruction;
  [key: string | symbol]: unknown;
}

export type DropOperation = "ReorderGlobals" | "ReorderGrouped" | "MoveToGlobal" | "MoveToGrouped" | "CombineToGrouped";
