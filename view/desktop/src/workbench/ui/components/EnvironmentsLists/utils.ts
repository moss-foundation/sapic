import { extractInstruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { StreamEnvironmentsEvent } from "@repo/ipc";

import { ENVIRONMENT_ITEM_DRAG_TYPE, ENVIRONMENT_LIST_DRAG_TYPE } from "./constants";
import {
  DragEnvironmentItem,
  DropEnvironmentItem,
  DropOperation,
  GlobalEnvironmentItem,
  GroupedEnvironmentItem,
  GroupedEnvironmentList,
  GroupedEnvironments,
} from "./types";

//source
export const getSourceEnvironmentItem = (source: ElementDragPayload): DragEnvironmentItem | null => {
  if (
    source.data.type !== ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL &&
    source.data.type !== ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED
  ) {
    return null;
  }

  return source.data as unknown as DragEnvironmentItem;
};

export const isSourceEnvironmentItem = (source: ElementDragPayload): boolean => {
  return (
    source.data.type === ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL || source.data.type === ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED
  );
};

export const isSourceGlobalEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL;
};

export const isSourceGroupedEnvironmentItem = (source: ElementDragPayload): boolean => {
  return source.data.type === ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED;
};

export const getSourceGlobalEnvironmentItemData = (source: ElementDragPayload): GlobalEnvironmentItem | null => {
  if (source.data.type !== ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL) {
    return null;
  }

  return source.data as unknown as GlobalEnvironmentItem;
};

export const isSourceGroupedEnvironmentList = (source: ElementDragPayload): boolean => {
  return source.data.type === ENVIRONMENT_LIST_DRAG_TYPE.GROUPED;
};

export const getSourceGroupedEnvironmentItemData = (source: ElementDragPayload): GroupedEnvironmentItem | null => {
  if (source.data.type !== ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED) {
    return null;
  }

  return source.data as unknown as GroupedEnvironmentItem;
};

export const getSourceGroupedEnvironmentListData = (source: ElementDragPayload): GroupedEnvironmentList | null => {
  if (source.data.type !== ENVIRONMENT_LIST_DRAG_TYPE.GROUPED) {
    return null;
  }

  return source.data as unknown as GroupedEnvironmentList;
};

//location
export const isLocationGroupedEnvironmentList = (location: DragLocationHistory): boolean => {
  if (location.current.dropTargets.length === 0 || location.current.dropTargets.length > 1) return false;
  return location.current.dropTargets[0].data.type === ENVIRONMENT_LIST_DRAG_TYPE.GROUPED;
};

export const isLocationGlobalEnvironmentItem = (location: DragLocationHistory): boolean => {
  return location.current.dropTargets[0].data.type === ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL;
};

export const isLocationGroupedEnvironmentItem = (location: DragLocationHistory): boolean => {
  return location.current.dropTargets[0].data.type === ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED;
};

export const getLocationEnvironmentItemData = (location: DragLocationHistory): DropEnvironmentItem | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (
    location.current.dropTargets[0].data.type !== ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL &&
    location.current.dropTargets[0].data.type !== ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED
  )
    return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    "data": {
      ...(location.current.dropTargets[0].data.data as DropEnvironmentItem["data"]),
    },
    "type": location.current.dropTargets[0].data.type,
    "instruction": instruction ?? undefined,
  };
};

export const getLocationGlobalEnvironmentItemData = (location: DragLocationHistory): GlobalEnvironmentItem | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL) return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    "data": {
      ...(location.current.dropTargets[0].data.data as GlobalEnvironmentItem["data"]),
    },
    "type": location.current.dropTargets[0].data.type,
    "instruction": instruction ?? undefined,
  };
};

export const getLocationGroupedEnvironmentItemData = (location: DragLocationHistory): GroupedEnvironmentItem | null => {
  if (location.current.dropTargets.length === 0) return null;
  if (location.current.dropTargets[0].data.type !== ENVIRONMENT_ITEM_DRAG_TYPE.GROUPED) return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    "data": {
      ...(location.current.dropTargets[0].data.data as GroupedEnvironmentItem["data"]),
    },
    "type": location.current.dropTargets[0].data.type,
    "instruction": instruction ?? undefined,
  };
};

export const getLocationGroupedEnvironmentListData = (location: DragLocationHistory): GroupedEnvironmentList | null => {
  if (location.current.dropTargets.length === 0 || location.current.dropTargets.length > 1) return null;
  if (location.current.dropTargets[0].data.type !== ENVIRONMENT_LIST_DRAG_TYPE.GROUPED) return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    "data": {
      ...(location.current.dropTargets[0].data.data as GroupedEnvironmentList["data"]),
    },
    "type": location.current.dropTargets[0].data.type,
    "instruction": instruction ?? undefined,
  };
};

//other

export const getDropOperation = (source: ElementDragPayload, location: DragLocationHistory): DropOperation | null => {
  if (location.current.dropTargets.length === 0) return null;

  const instruction = extractInstruction(location.current.dropTargets[0].data);

  if (!instruction || instruction.blocked) {
    return null;
  }

  if (instruction.operation === "combine") {
    if (isLocationGroupedEnvironmentList(location)) {
      return "CombineToGrouped";
    } else {
      return null;
    }
  }

  if (isSourceGlobalEnvironmentItem(source) && isLocationGlobalEnvironmentItem(location)) {
    return "ReorderGlobals";
  }

  if (isSourceGlobalEnvironmentItem(source) && isLocationGroupedEnvironmentItem(location)) {
    return "MoveToGrouped";
  }

  if (isSourceGroupedEnvironmentItem(source) && isLocationGroupedEnvironmentItem(location)) {
    const sourceData = getSourceGroupedEnvironmentItemData(source);
    const locationData = getLocationGroupedEnvironmentItemData(location);
    if (!sourceData || !locationData) {
      return null;
    }

    if (sourceData.data.environment.projectId !== locationData.data.environment.projectId) {
      return "MoveToGrouped";
    }

    return "ReorderGrouped";
  }

  if (isSourceGroupedEnvironmentItem(source) && isLocationGlobalEnvironmentItem(location)) {
    return "MoveToGlobal";
  }

  return null;
};

export const hasSimilarEnv = (groupedEnv: GroupedEnvironments, env: StreamEnvironmentsEvent): boolean => {
  return groupedEnv.environments.some(
    (groupedEnv) => groupedEnv.name.toLowerCase() === env.name.toLowerCase() || groupedEnv.id === env.id
  );
};
