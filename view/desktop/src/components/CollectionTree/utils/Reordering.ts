import { DragNode, DropNode } from "../types";

export const combineNodesForTheSameCollection = async (
  sourceTreeNodeData: DragNode,
  locationTreeNodeData: DropNode,
  updateCollectionEntry: (args: { collectionId: string; updatedEntry: UpdateCollectionEntryInput }) => Promise<void>
) => {
  const newOrder = locationTreeNodeData.node.childNodes.length + 1;

  if (sourceTreeNodeData.node.kind === "Dir") {
    await updateCollectionEntry({
      collectionId: sourceTreeNodeData.collectionId,
      updatedEntry: {
        DIR: {
          id: sourceTreeNodeData.node.id,
          path: locationTreeNodeData.parentNode.path.raw,
          order: newOrder,
        },
      },
    });
  } else {
    await updateCollectionEntry({
      collectionId: sourceTreeNodeData.collectionId,
      updatedEntry: {
        ITEM: {
          id: sourceTreeNodeData.node.id,
          path: locationTreeNodeData.parentNode.path.raw,
          order: newOrder,
        },
      },
    });
  }

  await fetchEntriesForPath(sourceTreeNodeData.collectionId, locationTreeNodeData.parentNode.path.raw);
};
