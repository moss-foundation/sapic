import { useCallback, useEffect } from "react";

import { useDeleteCollectionEntry } from "@/hooks";
import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useBatchCreateCollectionEntry } from "@/hooks/collection/useBatchCreateCollectionEntry";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { useCreateCollectionEntry } from "@/hooks/collection/useCreateCollectionEntry";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { BatchUpdateEntryKind } from "@repo/moss-collection";
import { join } from "@tauri-apps/api/path";

import { DragNode, DropNode, DropRootNode } from "../types";
import {
  canDropNode,
  createEntryKind,
  getAllNestedEntries,
  getInstructionFromLocation,
  getLocationTreeCollectionNodeData,
  getLocationTreeRootNodeData,
  getPathWithoutName,
  getSourceTreeCollectionNodeData,
  hasAnotherDirectDescendantWithSimilarName,
  isSourceTreeCollectionNode,
  prepareEntriesForCreation,
  prepareEntriesForDrop,
  sortByOrder,
} from "../utils";

export const useNodeDragAndDropHandler = () => {
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();
  const { mutateAsync: deleteCollectionEntry } = useDeleteCollectionEntry();

  const { mutateAsync: batchCreateCollectionEntry } = useBatchCreateCollectionEntry();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();

  const { fetchEntriesForPath } = useFetchEntriesForPath();

  const handleCombineWithinCollection = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode) => {
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
              queryParamsToAdd: [],
              queryParamsToUpdate: [],
              queryParamsToRemove: [],
              pathParamsToAdd: [],
              pathParamsToUpdate: [],
              pathParamsToRemove: [],
              headersToAdd: [],
              headersToUpdate: [],
              headersToRemove: [],
            },
          },
        });
      }

      await fetchEntriesForPath(locationTreeNodeData.collectionId, locationTreeNodeData.parentNode.path.raw);

      return;
    },
    [updateCollectionEntry, fetchEntriesForPath]
  );

  const handleReorderWithinCollection = useCallback(
    async (
      sourceTreeNodeData: DragNode,
      locationTreeNodeData: DropNode,
      operation: "reorder-before" | "reorder-after" | "combine"
    ) => {
      const dropOrder =
        operation === "reorder-before"
          ? locationTreeNodeData.node.order! - 0.5
          : locationTreeNodeData.node.order! + 0.5;

      const sortedParentNodes = sortByOrder([...locationTreeNodeData.parentNode.childNodes]);
      const parentNodesWithNewOrders = [
        ...sortedParentNodes.slice(0, dropOrder).filter((entry) => entry.id !== sourceTreeNodeData.node.id),
        sourceTreeNodeData.node,
        ...sortedParentNodes.slice(dropOrder).filter((entry) => entry.id !== sourceTreeNodeData.node.id),
      ].map((entry, index) => ({
        ...entry,
        order: index + 1,
      }));

      const updatedParentNodes = parentNodesWithNewOrders.filter((node) => {
        const nodeInLocation = locationTreeNodeData.parentNode.childNodes.find((n) => n.id === node.id);
        return nodeInLocation?.order !== node.order;
      });

      const parentEntriesToUpdate = updatedParentNodes.map((entry): BatchUpdateEntryKind => {
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
              queryParamsToAdd: [],
              queryParamsToUpdate: [],
              queryParamsToRemove: [],
              pathParamsToAdd: [],
              pathParamsToUpdate: [],
              pathParamsToRemove: [],
              headersToAdd: [],
              headersToUpdate: [],
              headersToRemove: [],
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

      await fetchEntriesForPath(locationTreeNodeData.collectionId, locationTreeNodeData.parentNode.path.raw);
      await fetchEntriesForPath(sourceTreeNodeData.collectionId, sourceTreeNodeData.parentNode.path.raw);

      return;
    },
    [batchUpdateCollectionEntry, fetchEntriesForPath]
  );

  const handleCombineToAnotherCollectionRoot = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeRootNodeData: DropRootNode) => {
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
      const newOrder = locationTreeRootNodeData.node.requests.childNodes.length + 1;

      await deleteCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        input: { id: sourceTreeNodeData.node.id },
      });

      const batchCreateEntryInput = await Promise.all(
        entriesWithoutName.map(async (entry, index) => {
          const newEntryPath = await join(locationTreeRootNodeData.node.requests.path.raw, entry.path.raw);

          if (index === 0) {
            return createEntryKind({
              name: entry.name,
              path: locationTreeRootNodeData.node.requests.path.raw,
              isAddingFolder: entry.kind === "Dir",
              order: newOrder,
              protocol: entry.protocol,
            });
          } else {
            return createEntryKind({
              name: entry.name,
              path: newEntryPath,
              isAddingFolder: entry.kind === "Dir",
              order: entry.order!,
              protocol: entry.protocol,
            });
          }
        })
      );

      await batchCreateCollectionEntry({
        collectionId: locationTreeRootNodeData.collectionId,
        input: {
          entries: batchCreateEntryInput,
        },
      });

      await fetchEntriesForPath(locationTreeRootNodeData.collectionId, locationTreeRootNodeData.node.requests.path.raw);
      await fetchEntriesForPath(sourceTreeNodeData.collectionId, sourceTreeNodeData.parentNode.path.raw);
    },
    [deleteCollectionEntry, batchCreateCollectionEntry, fetchEntriesForPath]
  );

  const handleMoveToAnotherCollection = useCallback(
    async (
      sourceTreeNodeData: DragNode,
      locationTreeNodeData: DropNode,
      operation: "reorder-before" | "reorder-after" | "combine"
    ) => {
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

      const dropOrder =
        operation === "reorder-before"
          ? locationTreeNodeData.node.order! - 0.5
          : locationTreeNodeData.node.order! + 0.5;

      const sortedParentNodes = sortByOrder(locationTreeNodeData.parentNode.childNodes);

      const dropParentNodesWithNewOrders = [
        ...sortedParentNodes.slice(0, dropOrder),
        sourceTreeNodeData.node,
        ...sortedParentNodes.slice(dropOrder),
      ].map((entry, index) => ({ ...entry, order: index + 1 }));
      const newOrder = dropParentNodesWithNewOrders.findIndex((entry) => entry.id === sourceTreeNodeData.node.id) + 1;

      const dropParentEntriesToUpdate = dropParentNodesWithNewOrders
        .slice(dropOrder + 1)
        .map((entry): BatchUpdateEntryKind => {
          if (entry.kind === "Dir") {
            return {
              DIR: {
                id: entry.id,
                order: entry.order,
              },
            };
          } else {
            return {
              ITEM: {
                id: entry.id,
                order: entry.order,
                queryParamsToAdd: [],
                queryParamsToUpdate: [],
                queryParamsToRemove: [],
                pathParamsToAdd: [],
                pathParamsToUpdate: [],
                pathParamsToRemove: [],
                headersToAdd: [],
                headersToUpdate: [],
                headersToRemove: [],
              },
            };
          }
        });

      const entriesAfterDeletedNodesWithUpdatedOrders = sourceTreeNodeData.parentNode.childNodes
        .filter((entry) => entry.order! > sourceTreeNodeData.node.order!)
        .map((entry): BatchUpdateEntryKind => {
          if (entry.kind === "Dir") {
            return {
              DIR: {
                id: entry.id,
                order: entry.order! - 1,
              },
            };
          } else {
            return {
              ITEM: {
                id: entry.id,
                order: entry.order! - 1,
                queryParamsToAdd: [],
                queryParamsToUpdate: [],
                queryParamsToRemove: [],
                pathParamsToAdd: [],
                pathParamsToUpdate: [],
                pathParamsToRemove: [],
                headersToAdd: [],
                headersToUpdate: [],
                headersToRemove: [],
              },
            };
          }
        });
      await batchUpdateCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        entries: {
          entries: entriesAfterDeletedNodesWithUpdatedOrders,
        },
      });

      await batchUpdateCollectionEntry({
        collectionId: locationTreeNodeData.collectionId,
        entries: {
          entries: dropParentEntriesToUpdate,
        },
      });

      await deleteCollectionEntry({
        collectionId: sourceTreeNodeData.collectionId,
        input: { id: sourceTreeNodeData.node.id },
      });

      const batchCreateEntryInput = await Promise.all(
        entriesWithoutName.map(async (entry, index) => {
          if (index === 0) {
            return createEntryKind({
              name: entry.name,
              path: locationTreeNodeData.parentNode.path.raw,
              isAddingFolder: entry.kind === "Dir",
              order: newOrder,
              protocol: entry.protocol,
            });
          } else {
            const newEntryPath = await join(locationTreeNodeData.parentNode.path.raw, entry.path.raw);
            return createEntryKind({
              name: entry.name,
              path: newEntryPath,
              isAddingFolder: entry.kind === "Dir",
              order: entry.order!,
              protocol: entry.protocol,
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

      await fetchEntriesForPath(locationTreeNodeData.collectionId, locationTreeNodeData.parentNode.path.raw);
      await fetchEntriesForPath(sourceTreeNodeData.collectionId, sourceTreeNodeData.parentNode.path.raw);
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

      const batchCreateEntryInput = await Promise.all(
        entriesPreparedForCreation.map(async (entry, index) => {
          if (index === 0) {
            return createEntryKind(
              entry.name,
              locationTreeNodeData.node.path.raw,
              entry.kind === "Dir",
              entry.class,
              newOrder
            );
          } else {
            const newEntryPath = await join(locationTreeNodeData.node.path.raw, entry.path.raw);
            return createEntryKind(entry.name, newEntryPath, entry.kind === "Dir", entry.class, entry.order!);
          }
        })
      );

      await batchCreateCollectionEntry({
        collectionId: locationTreeNodeData.collectionId,
        input: { entries: batchCreateEntryInput },
      });

      await fetchEntriesForPath(locationTreeNodeData.collectionId, locationTreeNodeData.parentNode.path.raw);
      await fetchEntriesForPath(sourceTreeNodeData.collectionId, sourceTreeNodeData.parentNode.path.raw);

      return;
    },
    [batchCreateCollectionEntry, deleteCollectionEntry, fetchEntriesForPath]
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

        if (instruction?.blocked) {
          console.warn("can't drop: blocked");
          return;
        }

        if (locationTreeRootNodeData && operation === "combine") {
          if (
            hasAnotherDirectDescendantWithSimilarName(locationTreeRootNodeData.node.requests, sourceTreeNodeData.node)
          ) {
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

        if (!canDropNode(sourceTreeNodeData, locationTreeNodeData) || !operation) {
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
