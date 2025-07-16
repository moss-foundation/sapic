import { useEffect } from "react";

import { useDeleteCollectionEntry } from "@/hooks";
import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { useCreateCollectionEntry } from "@/hooks/collection/useCreateCollectionEntry";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

import { getPathWithoutName, prepareEntriesForDrop } from "../utils/Path";
import {
  createEntry,
  doesLocationHaveTreeNode,
  getAllNestedEntries,
  getInstructionFromLocation,
  getLocationTreeNodeData,
  getSourceTreeNodeData,
  isSourceTreeNode,
  sortByOrder,
} from "../utils2";

export const useMonitorForNodeDragAndDrop = () => {
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: deleteCollectionEntry } = useDeleteCollectionEntry();
  const { fetchEntriesForPath } = useFetchEntriesForPath();

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceTreeNode(source);
      },
      onDrop: async ({ location, source }) => {
        if (!isSourceTreeNode(source) || !doesLocationHaveTreeNode(location)) {
          return;
        }

        const sourceTreeNodeData = getSourceTreeNodeData(source);
        const locationTreeNodeData = getLocationTreeNodeData(location);
        const operation = getInstructionFromLocation(location)?.operation;

        if (sourceTreeNodeData.node.id === locationTreeNodeData.node.id || !operation) {
          return;
        }

        if (sourceTreeNodeData.collectionId === locationTreeNodeData.collectionId) {
          if (operation === "combine") {
            const newOrder = locationTreeNodeData.node.childNodes.length + 1;

            if (sourceTreeNodeData.node.kind === "Dir") {
              await updateCollectionEntry({
                collectionId: sourceTreeNodeData.collectionId,
                updatedEntry: {
                  DIR: {
                    id: sourceTreeNodeData.node.id,
                    path: locationTreeNodeData.node.path.raw,
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
                    path: locationTreeNodeData.node.path.raw,
                    order: newOrder,
                  },
                },
              });
            }

            await fetchEntriesForPath(sourceTreeNodeData.collectionId, locationTreeNodeData.parentNode.path.raw);

            return;
          } else {
            const dropOrder =
              operation === "reorder-before"
                ? locationTreeNodeData.node.order! - 0.5
                : locationTreeNodeData.node.order! + 0.5;

            const sortedParentNodes = sortByOrder([...locationTreeNodeData.parentNode.childNodes]).filter(
              (entry) => entry.id !== sourceTreeNodeData.node.id
            );

            const parentNodesWithNewOrders = [
              ...sortedParentNodes.slice(0, dropOrder),
              sourceTreeNodeData.node,
              ...sortedParentNodes.slice(dropOrder),
            ].map((entry, index) => ({
              ...entry,
              order: index + 1,
            }));

            const updatedParentNodes = parentNodesWithNewOrders.filter((node) => {
              const nodeInLocation = locationTreeNodeData.parentNode.childNodes.find((n) => n.id === node.id);
              return nodeInLocation?.order !== node.order;
            });

            const parentEntriesToUpdate = updatedParentNodes.map((entry) => {
              const isAlreadyInLocation = locationTreeNodeData.parentNode.childNodes.some((n) => n.id === entry.id);
              const newEntryPath = isAlreadyInLocation ? undefined : locationTreeNodeData.parentNode.path.raw;

              if (entry.kind === "Dir") {
                return {
                  DIR: {
                    id: entry.id,
                    order: entry.order,
                    path: newEntryPath,
                  },
                };
              } else {
                return {
                  ITEM: {
                    id: entry.id,
                    order: entry.order,
                    path: newEntryPath,
                  },
                };
              }
            });

            await batchUpdateCollectionEntry({
              collectionId: sourceTreeNodeData.collectionId,
              entries: {
                entries: parentEntriesToUpdate,
              },
            });

            await fetchEntriesForPath(sourceTreeNodeData.collectionId, locationTreeNodeData.parentNode.path.raw);
            await fetchEntriesForPath(sourceTreeNodeData.collectionId, sourceTreeNodeData.parentNode.path.raw);

            return;
          }
        } else {
          const allEntries = getAllNestedEntries(sourceTreeNodeData.node);
          const entriesPreparedForDrop = await prepareEntriesForDrop(allEntries);
          const entriesWithoutName = await Promise.all(
            entriesPreparedForDrop.map(async (entry) => {
              const pathWithoutName = await getPathWithoutName(entry);

              return {
                ...entry,
                path: pathWithoutName,
              };
            })
          );

          if (operation === "combine") {
            const newOrder = locationTreeNodeData.node.childNodes.length + 1;

            await deleteCollectionEntry({
              collectionId: sourceTreeNodeData.collectionId,
              input: { id: sourceTreeNodeData.node.id },
            });

            await Promise.all(
              entriesWithoutName.map(async (entry, index) => {
                const newEntryPath = await join(locationTreeNodeData.node.path.raw, entry.path.raw);

                if (index === 0) {
                  await createCollectionEntry({
                    collectionId: locationTreeNodeData.collectionId,
                    input: createEntry(entry.name, locationTreeNodeData.node.path.raw, entry.kind === "Dir", newOrder),
                  });
                } else {
                  await createCollectionEntry({
                    collectionId: locationTreeNodeData.collectionId,
                    input: createEntry(entry.name, newEntryPath, entry.kind === "Dir", entry.order!),
                  });
                }
              })
            );
          } else {
          }
        }
      },
    });
  }, [
    batchUpdateCollectionEntry,
    createCollectionEntry,
    deleteCollectionEntry,
    fetchEntriesForPath,
    updateCollectionEntry,
  ]);
};
