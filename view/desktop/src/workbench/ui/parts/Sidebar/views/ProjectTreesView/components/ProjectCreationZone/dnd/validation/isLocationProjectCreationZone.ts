import { DragLocationHistory } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { PROJECT_CREATION_ZONE_TYPE } from "../../constants";

export const isLocationProjectCreationZone = (location: DragLocationHistory): boolean => {
  return (
    location.current.dropTargets.length > 0 && location.current.dropTargets[0].data.type === PROJECT_CREATION_ZONE_TYPE
  );
};
