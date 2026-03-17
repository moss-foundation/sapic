import { DragResourceNode, DragResourceNodeData } from "../types.dnd";

interface HasDirectDescendantWithSimilarNameProps {
  sourceData: DragResourceNodeData;
  locationData: DragResourceNode;
}

export const hasDirectDescendantWithSimilarName = ({
  sourceData,
  locationData,
}: HasDirectDescendantWithSimilarNameProps): boolean => {
  if (locationData.data.node.childNodes.length === 0) return false;

  return locationData.data.node.childNodes.some(
    (child) => child.id === sourceData.node.id || child.name.toLowerCase() === sourceData.node.name.toLowerCase()
  );
};
