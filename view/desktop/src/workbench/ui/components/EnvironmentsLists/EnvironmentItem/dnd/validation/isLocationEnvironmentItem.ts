import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../../../constants";

export const isLocationEnvironmentItem = (location: DragLocationHistory): boolean => {
  return (
    location.current.dropTargets[0].data.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ||
    location.current.dropTargets[0].data.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE
  );
};
