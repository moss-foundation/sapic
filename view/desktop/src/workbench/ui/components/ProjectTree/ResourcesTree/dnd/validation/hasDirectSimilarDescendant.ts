import { DraggedResourceNode } from "../../../types";
import { DragResourceNode } from "../types.dnd";

interface HasDirectSimilarDescendantProps {
  locationData: DragResourceNode;
  sourceData: DraggedResourceNode;
}

export const hasDirectSimilarDescendant = ({ locationData, sourceData }: HasDirectSimilarDescendantProps): boolean => {
  if (locationData.data.node.childNodes.length === 0) return false;

  return locationData.data.node.childNodes.some(
    (child) => child.id === sourceData.node.id || child.name.toLowerCase() === sourceData.node.name.toLowerCase()
  );
};
