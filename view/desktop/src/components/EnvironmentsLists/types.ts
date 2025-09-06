import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { EnvironmentGroup, StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface GroupedEnvironments extends EnvironmentGroup {
  environments: StreamEnvironmentsEvent[];
}

export type EnvironmentListType = "GlobalEnvironmentItem" | "GroupedEnvironmentItem";

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

export type DropOperation =
  | "GlobalReorder"
  | "GroupedReorder"
  | "GlobalCombine"
  | "GroupedCombine"
  | "MoveToGlobal"
  | "MoveToGrouped";
