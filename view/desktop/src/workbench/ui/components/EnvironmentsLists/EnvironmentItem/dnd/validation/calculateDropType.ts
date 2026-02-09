import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../../../constants";
import { DragEnvironmentItem, DropEnvironmentItem, EnvironmentsDropOperations } from "../types.dnd";

export const calculateDropType = (
  sourceData: DragEnvironmentItem,
  locationData: DropEnvironmentItem
): EnvironmentsDropOperations => {
  const instruction = extractInstruction(locationData);

  if (!instruction) {
    console.warn("Invalid instruction for environments lists", { instruction });
    return null;
  }

  if (
    sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE &&
    locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE
  ) {
    return "ReorderWorkspaceEnvs";
  }

  if (
    sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT &&
    locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT
  ) {
    if (sourceData.data.projectId === locationData.data.projectId) {
      return "ReorderProjectEnvs";
    } else {
      return "MoveProjectEnvToProjectEnv";
    }
  }

  if (
    sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE &&
    locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT
  ) {
    return "MoveWorkspaceEnvToProjectEnvs";
  }

  if (
    sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT &&
    locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE
  ) {
    return "MoveProjectEnvToWorkspaceEnvs";
  }

  //combine to environment project list

  return null;
};
