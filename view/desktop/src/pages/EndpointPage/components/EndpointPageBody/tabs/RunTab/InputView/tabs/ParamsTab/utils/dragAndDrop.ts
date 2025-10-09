import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { DragLocationHistory, ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import {
  DRAGGABLE_PARAM_ROW_TYPE,
  DROP_TARGET_NEW_PARAM_ROW_FORM_TYPE,
  DROP_TARGET_PARAM_ROW_TYPE,
} from "../constants";
import { DraggableParamRowData, DropTargetNewParamRowFormData, DropTargetParamRowData } from "../types";

//source
export const isSourceParamRow = (source: ElementDragPayload) => {
  return source.data.type === DRAGGABLE_PARAM_ROW_TYPE;
};

export const isSourceQueryParamRow = (source: ElementDragPayload) => {
  if (!isSourceParamRow(source)) return false;
  const data = source.data as DraggableParamRowData;
  return data.data.paramType === "query";
};

export const isSourcePathParamRow = (source: ElementDragPayload) => {
  if (!isSourceParamRow(source)) return false;
  const data = source.data as DraggableParamRowData;
  return data.data.paramType === "path";
};

export const getDraggableParamRowSourceData = (source: ElementDragPayload): DraggableParamRowData | null => {
  if (!isSourceParamRow(source)) return null;

  const data = source.data as DraggableParamRowData;
  return data;
};

//location
export const isLocationParamRow = (location: DragLocationHistory) => {
  if (location.current.dropTargets.length === 0) return false;
  if (location.current.dropTargets[0].data.type !== DROP_TARGET_PARAM_ROW_TYPE) return false;
  return true;
};

export const getFirstDraggableParamRowLocationData = (location: DragLocationHistory): DropTargetParamRowData | null => {
  if (!isLocationParamRow(location)) return null;
  const edge = extractClosestEdge(location.current.dropTargets[0].data);
  return { ...location.current.dropTargets[0].data, edge } as unknown as DropTargetParamRowData;
};

export const getAllDraggableParamRowLocationData = (location: DragLocationHistory): DropTargetParamRowData[] | null => {
  if (!isLocationParamRow(location)) return null;
  return location.current.dropTargets.map((target) => target.data as unknown as DropTargetParamRowData);
};

// location NewParamRowForm
export const isLocationNewParamRowForm = (location: DragLocationHistory) => {
  if (location.current.dropTargets.length === 0) return false;
  if (location.current.dropTargets[0].data.type !== DROP_TARGET_NEW_PARAM_ROW_FORM_TYPE) return false;
  return true;
};

export const getFirstNewParamRowFormLocationData = (
  location: DragLocationHistory
): DropTargetNewParamRowFormData | null => {
  if (!isLocationNewParamRowForm(location)) return null;
  return location.current.dropTargets[0].data as unknown as DropTargetNewParamRowFormData;
};

//rest
export const calculateDropType = (
  sourceData: DraggableParamRowData,
  dropTargetData: DropTargetParamRowData | DropTargetNewParamRowFormData
) => {
  if (sourceData.data.paramType === "query" && dropTargetData.data.paramType === "query") {
    return "WithinQueryList";
  }

  if (sourceData.data.paramType === "path" && dropTargetData.data.paramType === "path") {
    return "WithinPathList";
  }

  if (sourceData.data.paramType === "query" && dropTargetData.data.paramType === "path") {
    return "QueryToPath";
  }

  if (sourceData.data.paramType === "path" && dropTargetData.data.paramType === "query") {
    return "PathToQuery";
  }

  return "Invalid";
};
