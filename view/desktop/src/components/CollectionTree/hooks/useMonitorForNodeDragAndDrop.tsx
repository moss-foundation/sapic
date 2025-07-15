import { useEffect } from "react";

import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { EntryInfo } from "@repo/moss-collection";
import { join } from "@tauri-apps/api/path";

import { TreeCollectionNode } from "../types";
import {
  doesLocationHaveTreeNode,
  getInstructionFromLocation,
  getLocationTreeNodeData,
  getSourceTreeNodeData,
  isSourceTreeNode,
  sortByOrder,
} from "../utils2";

export const useMonitorForNodeDragAndDrop = () => {
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();
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
        }
      },
    });
  }, [batchUpdateCollectionEntry, fetchEntriesForPath, updateCollectionEntry]);
};

export const getPathWithoutName = async (node: TreeCollectionNode | EntryInfo): Promise<EntryInfo["path"]> => {
  const newSegments = node.path.segments.filter((segment) => segment !== node.name);
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const getPathWithoutParentPath = async (
  path: EntryInfo["path"],
  parentPath: EntryInfo["path"]
): Promise<EntryInfo["path"]> => {
  const newSegments = path.segments.filter((segment) => !parentPath.segments.includes(segment));
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const removePathBeforeName = async (path: EntryInfo["path"], name: string) => {
  const nameIndex = path.segments.findIndex((segment) => segment === name);

  if (nameIndex === -1) {
    return {
      segments: path.segments,
      raw: path.raw,
    };
  }

  const newSegments = path.segments.slice(nameIndex);
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const prepareEntriesForDrop = async (entries: EntryInfo[]): Promise<EntryInfo[]> => {
  const rootEntryName = entries[0].name;

  const entriesPreparedForDrop: EntryInfo[] = [];

  for await (const entry of entries) {
    const newEntryPath = await removePathBeforeName(entry.path, rootEntryName);

    entriesPreparedForDrop.push({
      ...entry,
      path: newEntryPath,
    });
  }

  return entriesPreparedForDrop;
};
