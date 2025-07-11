import { useEffect } from "react";

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

        if (sourceTreeNodeData.collectionId === locationTreeNodeData.collectionId) {
          const allEntries = getAllNestedEntries(sourceTreeNodeData.node);

          if (locationTreeNodeData.instruction?.operation === "combine") {
            const newOrder = locationTreeNodeData.node.childNodes.length + 1;

            if (sourceTreeNodeData.node.kind === "Item") {
              await updateCollectionEntry({
                collectionId: sourceTreeNodeData.collectionId,
                updatedEntry: {
                  ITEM: {
                    id: sourceTreeNodeData.node.id,
                    path: await join(...locationTreeNodeData.node.path.segments, sourceTreeNodeData.node.name),
                    order: newOrder,
                  },
                },
              });
            }
          }

          if (
            locationTreeNodeData.instruction?.operation === "reorder-before" ||
            locationTreeNodeData.instruction?.operation === "reorder-after"
          ) {
            const newOrder =
              typeof locationTreeNodeData.node.order === "number"
                ? locationTreeNodeData.instruction?.operation === "reorder-after"
                  ? locationTreeNodeData.node.order + 1
                  : locationTreeNodeData.instruction?.operation === "reorder-before"
                    ? locationTreeNodeData.node.order - 1
                    : 0
                : 0;

            console.log({
              sourceTreeNodeData,
              locationTreeNodeData,
            });

            const rootEntry = allEntries[0];
            const nestedEntries = allEntries.slice(1);

            if (rootEntry.kind === "Dir") {
              const pathWithoutName = await getPathWithoutName(locationTreeNodeData.node);
              const newRootEntryPath = await join(pathWithoutName.raw, rootEntry.name);

              await updateCollectionEntry({
                collectionId: sourceTreeNodeData.collectionId,
                updatedEntry: {
                  DIR: {
                    id: rootEntry.id,
                    path: newRootEntryPath,
                    order: newOrder,
                  },
                },
              });

              for (const entry of nestedEntries) {
                const newEntryPath = await getPathWithoutParentPath(entry.path, locationTreeNodeData.node.path);
                const entryKind = entry.kind === "Dir" ? "DIR" : "ITEM";

                await batchUpdateCollectionEntry({
                  collectionId: sourceTreeNodeData.collectionId,
                  entries: {
                    entries: [
                      {
                        [entryKind]: {
                          id: entry.id,
                          path: await join(newRootEntryPath, ...newEntryPath.segments),
                        },
                      },
                    ],
                  },
                });
              }
            } else {
              const parentPathWithoutName = await getPathWithoutName(locationTreeNodeData.node);
              const newRootEntryPath = await join(parentPathWithoutName.raw, rootEntry.name);

              await updateCollectionEntry({
                collectionId: sourceTreeNodeData.collectionId,
                updatedEntry: {
                  ITEM: {
                    id: rootEntry.id,
                    path: newRootEntryPath === rootEntry.path.raw ? undefined : newRootEntryPath,
                    order: newOrder,
                  },
                },
              });
            }
          }

          //   allNestedEntries.forEach(async (entry) => {
          //     const newPath = await join(pathWithoutName.raw, entry.path.raw);

          //    await  updateCollectionEntry({
          //       collectionId: sourceTreeNodeData.collectionId,
          //       updatedEntry: {
          //         DIR: {
          //           id: entry.id,
          //           path: newPath,
          //         },
          //       },
          //     });
          //   });

          return;
        }
      },
    });
  }, [batchUpdateCollectionEntry, updateCollectionEntry]);
};

const getPathWithoutName = async (node: TreeCollectionNode | EntryInfo): Promise<EntryInfo["path"]> => {
  const newSegments = node.path.segments.filter((segment) => segment !== node.name);
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

const getPathWithoutParentPath = async (
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
