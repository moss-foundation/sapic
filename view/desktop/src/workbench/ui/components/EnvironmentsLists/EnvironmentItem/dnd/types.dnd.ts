import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../../constants";

export type EnvironmentsDropOperations =
  | "ReorderWorkspaceEnvs"
  | "ReorderProjectEnvs"
  | "MoveWorkspaceEnvToProjectEnvs"
  | "MoveProjectEnvToWorkspaceEnvs"
  | "MoveProjectEnvToProjectEnv"
  | "CombineWorkspaceEnvToProjectList"
  | "CombineProjectEnvToProjectList"
  | null;

export interface DragEnvironmentItem {
  type: ENVIRONMENT_ITEM_DRAG_TYPE;
  data: EnvironmentSummary;
  [key: string | symbol]: unknown;
}

export interface DropEnvironmentItem extends DragEnvironmentItem {
  instruction?: Instruction;
}
