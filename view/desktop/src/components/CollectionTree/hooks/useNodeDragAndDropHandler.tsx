import { useCallback, useEffect } from "react";

import { useDeleteCollectionEntry } from "@/hooks";
import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useBatchCreateCollectionEntry } from "@/hooks/collection/useBatchCreateCollectionEntry";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { useCreateCollectionEntry } from "@/hooks/collection/useCreateCollectionEntry";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

import { DragNode, DropNode, DropRootNode } from "../types";
import {
  canDropNode,
  createEntryKind,
  getAllNestedEntries,
  getInstructionFromLocation,
  getLocationTreeCollectionNodeData,
  getLocationTreeRootNodeData,
  getSourceTreeCollectionNodeData,
  hasAnotherDirectDescendantWithSimilarName,
  isSourceTreeCollectionNode,
  makeDirUpdatePayload,
  makeItemUpdatePayload,
  prepareEntriesForCreation,
  prepareNestedDirEntriesForDrop,
  reorderedNodesForDifferentDirPayload,
  reorderedNodesForSameDirPayload,
  resolveParentPath,
  siblingsAfterRemovalPayload,
} from "../utils";

export const useNodeDragAndDropHandler = () => {
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();
  const { mutateAsync: deleteCollectionEntry } = useDeleteCollectionEntry();

  const { mutateAsync: batchCreateCollectionEntry } = useBatchCreateCollectionEntry();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();

  const { fetchEntriesForPath } = useFetchEntriesForPath();

  //Within Collection
  const handleCombineWithinCollection = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode) => {
      const newOrder = locationTreeNodeData.node.childNodes.length + 1;

      const sourceNodeUpdate =
        sourceTreeNodeData.node.kind === "Dir"
          ? makeDirUpdatePayload({
              id: sourceTreeNodeData.node.id,
              path: locationTreeNodeData.node.path.raw,
              order: newOrder,
            })
          : makeItemUpdatePayload({
              id: sourceTreeNodeData.node.id,
              path: locationTreeNodeData.node.path.raw,
              order: newOrder,
            });

      const sourceParentNodes = sourceTreeNodeData.parentNode.childNodes;
      const nodesToUpdate = siblingsAfterRemovalPayload({
        nodes: sourceParentNodes,
        removedNode: sourceTreeNodeData.node,
      });

      const allUpdates = [sourceNodeUpdate, ...nodesToUpdate];
      await batchUpdateCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        entries: {
          entries: allUpdates,
        },
      });

      await fetchEntriesForPath(locationTreeNodeData.collectionId, resolveParentPath(locationTreeNodeData.parentNode));

      await fetchEntriesForPath(sourceTreeNodeData.collectionId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [batchUpdateCollectionEntry, fetchEntriesForPath]
  );

  const handleReorderWithinCollection = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode, operation: Operation) => {
      const dropIndex =
        operation === "reorder-before"
          ? locationTreeNodeData.node.order! - 0.5
          : locationTreeNodeData.node.order! + 0.5;

      const inSameDir = sourceTreeNodeData.parentNode.id === locationTreeNodeData.parentNode.id;
      if (inSameDir) {
        const updatedSourceNodesPayload = reorderedNodesForSameDirPayload({
          nodes: sourceTreeNodeData.parentNode.childNodes,
          movedId: sourceTreeNodeData.node.id,
          moveToIndex: dropIndex,
        });

        await batchUpdateCollectionEntry({
          collectionId: sourceTreeNodeData.collectionId,
          entries: {
            entries: updatedSourceNodesPayload,
          },
        });

        await fetchEntriesForPath(sourceTreeNodeData.collectionId, resolveParentPath(sourceTreeNodeData.parentNode));

        return;
      }

      const targetEntriesToUpdate = reorderedNodesForDifferentDirPayload({
        node: locationTreeNodeData.parentNode,
        newNode: sourceTreeNodeData.node,
        moveToIndex: dropIndex,
      });

      const sourceEntriesToUpdate = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      const allEntriesToUpdate = [...targetEntriesToUpdate, ...sourceEntriesToUpdate];

      await batchUpdateCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        entries: {
          entries: allEntriesToUpdate,
        },
      });

      await fetchEntriesForPath(locationTreeNodeData.collectionId, resolveParentPath(locationTreeNodeData.parentNode));
      await fetchEntriesForPath(sourceTreeNodeData.collectionId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [batchUpdateCollectionEntry, fetchEntriesForPath]
  );

  //To Another Collection
  const handleMoveToAnotherCollection = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode, operation: Operation) => {
      const dropIndex =
        operation === "reorder-before"
          ? locationTreeNodeData.node.order! - 0.5
          : locationTreeNodeData.node.order! + 0.5;

      const targetEntriesToUpdate = reorderedNodesForDifferentDirPayload({
        node: locationTreeNodeData.parentNode,
        newNode: sourceTreeNodeData.node,
        moveToIndex: dropIndex,
      }).filter((entry) => {
        if ("ITEM" in entry) {
          return entry.ITEM.id !== sourceTreeNodeData.node.id;
        } else {
          return entry.DIR.id !== sourceTreeNodeData.node.id;
        }
      });

      const updatedSourceEntriesPayload = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      await batchUpdateCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        entries: {
          entries: updatedSourceEntriesPayload,
        },
      });

      await batchUpdateCollectionEntry({
        collectionId: locationTreeNodeData.collectionId,
        entries: {
          entries: targetEntriesToUpdate,
        },
      });

      await deleteCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        input: { id: sourceTreeNodeData.node.id },
      });

      const newDropOrder =
        operation === "reorder-before" ? locationTreeNodeData.node.order! : locationTreeNodeData.node.order! + 1;
      const allEntries = getAllNestedEntries(sourceTreeNodeData.node);
      const nestedEntriesWithoutName = await prepareNestedDirEntriesForDrop(allEntries);
      const batchCreateEntryInput = await Promise.all(
        nestedEntriesWithoutName.map(async (entry, index) => {
          if (index === 0) {
            return createEntryKind({
              name: entry.name,
              path: "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : "",
              isAddingFolder: entry.kind === "Dir",
              order: newDropOrder,
              protocol: entry.protocol,
              class: "Endpoint",
            });
          } else {
            const newEntryPath = await join(
              "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : "",
              entry.path.raw
            );
            return createEntryKind({
              name: entry.name,
              path: newEntryPath,
              isAddingFolder: entry.kind === "Dir",
              order: entry.order!,
              protocol: entry.protocol,
              class: "Endpoint",
            });
          }
        })
      );

      await batchCreateCollectionEntry({
        collectionId: locationTreeNodeData.collectionId,
        input: {
          entries: batchCreateEntryInput,
        },
      });

      await fetchEntriesForPath(
        locationTreeNodeData.collectionId,
        "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : ""
      );
      await fetchEntriesForPath(
        sourceTreeNodeData.collectionId,
        "path" in sourceTreeNodeData.parentNode ? sourceTreeNodeData.parentNode.path.raw : ""
      );
    },
    [batchUpdateCollectionEntry, deleteCollectionEntry, batchCreateCollectionEntry, fetchEntriesForPath]
  );

  const handleCombineToAnotherCollection = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode) => {
      const allEntries = getAllNestedEntries(sourceTreeNodeData.node);
      const entriesPreparedForCreation = await prepareEntriesForCreation(allEntries);

      const newOrder = locationTreeNodeData.node.childNodes.length + 1;

      await deleteCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        input: { id: sourceTreeNodeData.node.id },
      });

      const updatedSourceEntriesPayload = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      await batchUpdateCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        entries: {
          entries: updatedSourceEntriesPayload,
        },
      });

      const batchCreateEntryInput = await Promise.all(
        entriesPreparedForCreation.map(async (entry, index) => {
          if (index === 0) {
            return createEntryKind({
              name: entry.name,
              path: locationTreeNodeData.node.path.raw,
              isAddingFolder: entry.kind === "Dir",
              order: newOrder,
              protocol: entry.protocol,
              class: "Endpoint",
            });
          } else {
            const newEntryPath = await join(locationTreeNodeData.node.path.raw, entry.path.raw);
            return createEntryKind({
              name: entry.name,
              path: newEntryPath,
              isAddingFolder: entry.kind === "Dir",
              order: entry.order!,
              protocol: entry.protocol,
              class: "Endpoint",
            });
          }
        })
      );

      await batchCreateCollectionEntry({
        collectionId: locationTreeNodeData.collectionId,
        input: { entries: batchCreateEntryInput },
      });

      await fetchEntriesForPath(locationTreeNodeData.collectionId, resolveParentPath(locationTreeNodeData.parentNode));
      await fetchEntriesForPath(sourceTreeNodeData.collectionId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [batchCreateCollectionEntry, batchUpdateCollectionEntry, deleteCollectionEntry, fetchEntriesForPath]
  );

  //To Another Collection's Root
  const handleCombineToAnotherCollectionRoot = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeRootNodeData: DropRootNode) => {
      const allEntries = getAllNestedEntries(sourceTreeNodeData.node);
      const entriesWithoutName = await prepareNestedDirEntriesForDrop(allEntries);

      const newOrder = locationTreeRootNodeData.node.childNodes.length + 1;

      await deleteCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        input: { id: sourceTreeNodeData.node.id },
      });

      const updatedSourceEntriesPayload = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      const batchCreateEntryInput = await Promise.all(
        entriesWithoutName.map(async (entry, index) => {
          const newEntryPath = await join(entry.path.raw);

          if (index === 0) {
            return createEntryKind({
              name: entry.name,
              path: "",
              class: "Endpoint",
              isAddingFolder: entry.kind === "Dir",
              order: newOrder,
              protocol: entry.protocol,
            });
          } else {
            return createEntryKind({
              name: entry.name,
              path: newEntryPath,
              class: "Endpoint",
              isAddingFolder: entry.kind === "Dir",
              order: entry.order!,
              protocol: entry.protocol,
            });
          }
        })
      );

      await batchUpdateCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        entries: {
          entries: updatedSourceEntriesPayload,
        },
      });

      await batchCreateCollectionEntry({
        collectionId: locationTreeRootNodeData.collectionId,
        input: {
          entries: batchCreateEntryInput,
        },
      });

      await fetchEntriesForPath(locationTreeRootNodeData.collectionId, "");
      await fetchEntriesForPath(sourceTreeNodeData.collectionId, resolveParentPath(sourceTreeNodeData.parentNode));
    },
    [deleteCollectionEntry, batchUpdateCollectionEntry, batchCreateCollectionEntry, fetchEntriesForPath]
  );

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceTreeCollectionNode(source);
      },
      onDrop: async ({ location, source }) => {
        const sourceTreeNodeData = getSourceTreeCollectionNodeData(source);
        const locationTreeNodeData = getLocationTreeCollectionNodeData(location);
        const locationTreeRootNodeData = getLocationTreeRootNodeData(location);

        const instruction = getInstructionFromLocation(location);
        const operation = instruction?.operation;

        if (!sourceTreeNodeData) {
          console.warn("can't drop: no source");
          return;
        }

        if (instruction?.blocked || !operation) {
          console.warn("can't drop: blocked or no operation", { instruction, operation });
          return;
        }

        if (locationTreeRootNodeData && operation === "combine") {
          if (hasAnotherDirectDescendantWithSimilarName(locationTreeRootNodeData.node, sourceTreeNodeData.node)) {
            console.warn("can't drop: has direct similar descendant");
            return;
          }

          await handleCombineToAnotherCollectionRoot(sourceTreeNodeData, locationTreeRootNodeData);
          return;
        }

        if (!locationTreeNodeData) {
          console.warn("can't drop: no location");
          return;
        }

        if (!canDropNode(sourceTreeNodeData, locationTreeNodeData, operation)) {
          console.warn("can't drop: invalid operation");
          return;
        }

        const isSameCollection = sourceTreeNodeData.collectionId === locationTreeNodeData.collectionId;
        if (isSameCollection) {
          if (operation === "combine") {
            await handleCombineWithinCollection(sourceTreeNodeData, locationTreeNodeData);
          } else {
            await handleReorderWithinCollection(sourceTreeNodeData, locationTreeNodeData, operation);
          }
        } else {
          if (operation === "combine") {
            await handleCombineToAnotherCollection(sourceTreeNodeData, locationTreeNodeData);
          } else {
            await handleMoveToAnotherCollection(sourceTreeNodeData, locationTreeNodeData, operation);
          }
        }
      },
    });
  }, [
    batchCreateCollectionEntry,
    batchUpdateCollectionEntry,
    createCollectionEntry,
    deleteCollectionEntry,
    fetchEntriesForPath,
    handleCombineWithinCollection,
    handleMoveToAnotherCollection,
    handleReorderWithinCollection,
    handleCombineToAnotherCollectionRoot,
    updateCollectionEntry,
    handleCombineToAnotherCollection,
  ]);
};
