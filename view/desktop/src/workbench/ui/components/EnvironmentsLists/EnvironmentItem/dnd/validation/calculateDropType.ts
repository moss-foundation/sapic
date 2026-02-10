import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";

import { ENVIRONMENT_ITEM_DRAG_TYPE, EnvironmentsDropOperations } from "../../../constants";
import { DragEnvironmentItem, DropEnvironmentItem } from "../types.dnd";

export const calculateDropType = (
  sourceData: DragEnvironmentItem,
  locationData: DropEnvironmentItem
): EnvironmentsDropOperations | null => {
  const instruction = extractInstruction(locationData);

  if (!instruction) {
    console.warn("Invalid instruction for environments lists", { instruction });
    return null;
  }

  if (
    sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE &&
    locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE
  ) {
    return EnvironmentsDropOperations.ReorderWorkspaceEnvs;
  }

  if (
    sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT &&
    locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT
  ) {
    if (sourceData.data.projectId === locationData.data.projectId) {
      return EnvironmentsDropOperations.ReorderProjectEnvs;
    } else {
      return EnvironmentsDropOperations.MoveProjectEnvToProjectEnv;
    }
  }

  if (
    sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE &&
    locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT
  ) {
    return EnvironmentsDropOperations.MoveWorkspaceEnvToProjectEnvs;
  }

  if (
    sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT &&
    locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE
  ) {
    return EnvironmentsDropOperations.MoveProjectEnvToWorkspaceEnvs;
  }

  //combine to environment project list

  return null;
};
