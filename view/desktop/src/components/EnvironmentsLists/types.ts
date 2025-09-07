import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { EnvironmentGroup, StreamEnvironmentsEvent } from "@repo/moss-workspace";

export interface GroupedEnvironments extends EnvironmentGroup {
  environments: StreamEnvironmentsEvent[];
}

export interface GlobalEnvironmentItem {
  type: "GlobalEnvironmentItem";
  data: {
    environment: StreamEnvironmentsEvent;
  };
  instruction?: Instruction;
}

export interface GroupedEnvironmentItem {
  type: "GroupedEnvironmentItem";
  data: {
    environment: StreamEnvironmentsEvent;
  };
  instruction?: Instruction;
}

export interface GroupedEnvironmentList {
  type: "GroupedEnvironmentList";
  data: {
    groupWithEnvironments: GroupedEnvironments;
  };
}

export type EnvironmentListType = "GlobalEnvironmentItem" | "GroupedEnvironmentItem" | "GroupedEnvironmentList";

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

export type DropOperation = "ReorderGlobal" | "ReorderGrouped" | "MoveToGlobal" | "MoveToGrouped" | "CombineGrouped";
