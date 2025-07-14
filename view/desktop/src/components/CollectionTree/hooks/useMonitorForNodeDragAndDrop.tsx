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
  getAllNestedEntries,
  getLocationTreeNodeData,
  getSourceTreeNodeData,
  isSourceTreeNode,
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

        if (sourceTreeNodeData.node.id === locationTreeNodeData.node.id) return;

        const allEntries = getAllNestedEntries(sourceTreeNodeData.node);
        const entriesPreparedForDrop = await prepareEntriesForDrop(allEntries);

        // console.log({ entriesPreparedForDrop });

        if (sourceTreeNodeData.collectionId === locationTreeNodeData.collectionId) {
          if (locationTreeNodeData.instruction?.operation === "combine") {
            const newOrder = locationTreeNodeData.node.childNodes.length + 1;

            const entriesToUpdate = await Promise.all(
              entriesPreparedForDrop.map(async (entry, index) => {
                const newEntryPath = locationTreeNodeData.node.path.raw;

                if (index === 0) {
                  if (entry.kind === "Dir") {
                    return {
                      DIR: {
                        id: entry.id,
                        path: newEntryPath,
                        order: newOrder,
                      },
                    };
                  } else {
                    return {
                      ITEM: {
                        id: entry.id,
                        path: newEntryPath,
                        order: newOrder,
                      },
                    };
                  }
                }

                if (entry.kind === "Dir") {
                  return {
                    DIR: {
                      id: entry.id,
                      path: newEntryPath,
                    },
                  };
                } else {
                  return {
                    ITEM: {
                      id: entry.id,
                      path: newEntryPath,
                    },
                  };
                }
              })
            );

            console.log({ entriesToUpdate });

            await batchUpdateCollectionEntry({
              collectionId: sourceTreeNodeData.collectionId,
              entries: {
                entries: entriesToUpdate,
              },
            });

            await fetchEntriesForPath(sourceTreeNodeData.collectionId, locationTreeNodeData.node.path.raw);
            await fetchEntriesForPath(sourceTreeNodeData.collectionId, sourceTreeNodeData.node.path.raw);

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
