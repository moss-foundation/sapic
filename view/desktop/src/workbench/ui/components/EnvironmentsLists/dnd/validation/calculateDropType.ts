import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";

import { ENVIRONMENT_ITEM_DRAG_TYPE, ENVIRONMENT_LIST_DRAG_TYPE, EnvironmentsDropOperations } from "../../constants";
import {
  DragEnvironmentItem,
  DropEnvironmentItem,
  DropProjectEnvironmentList,
  DropWorkspaceEnvironmentList,
} from "../types.dnd";

// Aliases to the enums to reduce line length and visual noise
const Operations = EnvironmentsDropOperations;
const ItemType = ENVIRONMENT_ITEM_DRAG_TYPE;
const ListType = ENVIRONMENT_LIST_DRAG_TYPE;

export const calculateDropType = (
  source: DragEnvironmentItem,
  target: DropEnvironmentItem | DropProjectEnvironmentList | DropWorkspaceEnvironmentList
): EnvironmentsDropOperations | null => {
  const instruction = extractInstruction(target);

  if (!instruction) {
    console.warn("Invalid instruction for environments lists", { instruction });
    return null;
  }

  const isWorkspaceSource = source.type === ItemType.WORKSPACE;
  const isProjectSource = source.type === ItemType.PROJECT;

  const isSameProject =
    isProjectSource && target.type === ItemType.PROJECT && source.data.projectId === target.data.projectId;

  // --------------------------------------------------------
  // REORDER OPERATIONS
  // --------------------------------------------------------
  if (instruction.operation === "reorder-before" || instruction.operation === "reorder-after") {
    if (isWorkspaceSource && target.type === ItemType.WORKSPACE) return Operations.ReorderWorkspaceEnvs;
    if (isWorkspaceSource && target.type === ItemType.PROJECT) return Operations.MoveWorkspaceEnvToProjectEnvs;

    if (isProjectSource && target.type === ItemType.WORKSPACE) return Operations.MoveProjectEnvToWorkspaceEnvs;

    if (isProjectSource && target.type === ItemType.PROJECT) {
      return isSameProject ? Operations.ReorderProjectEnvs : Operations.MoveProjectEnvToProjectEnv;
    }
  }

  // --------------------------------------------------------
  // COMBINE OPERATIONS
  // --------------------------------------------------------
  if (instruction.operation === "combine") {
    if (isWorkspaceSource && target.type === ListType.PROJECT) return Operations.CombineWorkspaceEnvToProjectList;

    if (isProjectSource && target.type === ListType.WORKSPACE) return Operations.CombineProjectEnvToWorkspaceList;
    if (isProjectSource && target.type === ListType.PROJECT) return Operations.CombineProjectEnvToProjectList;
  }

  return null;
};
