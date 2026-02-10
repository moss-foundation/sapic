import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { ENVIRONMENT_LIST_DRAG_TYPE } from "../../constants";

export const isLocationWorkspaceEnvironmentList = (location: DragLocationHistory): boolean => {
  return location.current.dropTargets[0].data.type === ENVIRONMENT_LIST_DRAG_TYPE.WORKSPACE;
};
