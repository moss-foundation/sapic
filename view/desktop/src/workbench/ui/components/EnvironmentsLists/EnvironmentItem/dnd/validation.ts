import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ENVIRONMENT_ITEM_DRAG_TYPE, ENVIRONMENT_LIST_DRAG_TYPE } from "../../constants";

export const isSourceEnvironmentItem = (source: ElementDragPayload): boolean => {
  return (
    source.data.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT || source.data.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE
  );
};

export const hasSameNameOrId = (environments: EnvironmentSummary[], name: string, id: string): boolean => {
  return environments.some((env) => env.name === name || env.id === id);
};

export const isLocationEnvironmentItem = (location: DragLocationHistory): boolean => {
  return (
    location.current.dropTargets[0].data.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ||
    location.current.dropTargets[0].data.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE
  );
};

export const isLocationProjectEnvironmentList = (location: DragLocationHistory): boolean => {
  return location.current.dropTargets[0].data.type === ENVIRONMENT_LIST_DRAG_TYPE.PROJECT;
};
