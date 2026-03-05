import { DragNode } from "../../../types";
import { LocationResourcesListData } from "../types.dnd";

export const canCombineToResourcesList = (sourceData: DragNode | null, locationData: LocationResourcesListData) => {
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
