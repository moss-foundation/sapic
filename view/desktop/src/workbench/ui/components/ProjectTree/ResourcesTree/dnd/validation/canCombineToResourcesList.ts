import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DragResourceNodeData, LocationResourcesListData } from "../types.dnd";

interface CanCombineToResourcesListProps {
  sourceData: DragResourceNodeData | null;
  locationData: LocationResourcesListData;
}

export const canCombineToResourcesList = ({
  sourceData,
  locationData,
}: CanCombineToResourcesListProps): Availability => {
  if (!sourceData) {
    return "not-available";
  }

  const { rootResourcesNodes } = locationData.data;

  const hasSameId = rootResourcesNodes.some((node) => node.id === sourceData.node.id);
  const hasSimilarName = rootResourcesNodes.some(
    (node) => node.name.toLowerCase() === sourceData.node.name.toLowerCase()
  );

  if (hasSameId || hasSimilarName) {
    return "blocked";
  }

  return "available";
};
