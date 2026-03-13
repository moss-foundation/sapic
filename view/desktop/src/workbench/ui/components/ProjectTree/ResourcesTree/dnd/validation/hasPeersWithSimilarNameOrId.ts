import { DragResourceNode, DragResourceNodeData } from "../types.dnd";

interface HasPeersWithSimilarNameOrIdProps {
  sourceData: DragResourceNodeData;
  locationData: DragResourceNode;
}

export const hasPeersWithSimilarNameOrId = ({
  sourceData,
  locationData,
}: HasPeersWithSimilarNameOrIdProps): boolean => {
  if (locationData.data.parentNode.id === sourceData.parentNode.id) return false;
  if (locationData.data.parentNode.childNodes.length === 0) return false;

  return locationData.data.parentNode.childNodes.some(
    (child) => child.id === sourceData.node.id || child.name.toLowerCase() === sourceData.node.name.toLowerCase()
  );
};
