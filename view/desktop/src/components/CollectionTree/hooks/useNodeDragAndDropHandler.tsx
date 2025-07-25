import { useCallback, useEffect } from "react";

import { useDeleteCollectionEntry } from "@/hooks";
import { useFetchEntriesForPath } from "@/hooks/collection/derivedHooks/useFetchEntriesForPath";
import { useBatchCreateCollectionEntry } from "@/hooks/collection/useBatchCreateCollectionEntry";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { useCreateCollectionEntry } from "@/hooks/collection/useCreateCollectionEntry";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

import { DragNode, DropNode, DropRootNode } from "../types";
import { canDropNode } from "../utils";
import { getLocationTreeNodeData, getLocationTreeRootNodeData, getSourceTreeNodeData } from "../utils/DragAndDrop";
import { getPathWithoutName, prepareEntriesForDrop } from "../utils/Path";
import {
  createEntryKind,
  getAllNestedEntries,
  getInstructionFromLocation,
  isSourceTreeNode,
  sortByOrder,
} from "../utils/utils2";

export const useNodeDragAndDropHandler = () => {
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: batchCreateCollectionEntry } = useBatchCreateCollectionEntry();

  const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();

  const { mutateAsync: deleteCollectionEntry } = useDeleteCollectionEntry();

  const { fetchEntriesForPath } = useFetchEntriesForPath();

  //Within collection
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

      await fetchEntriesForPath(locationTreeNodeData.collectionId, locationTreeNodeData.parentNode.path.raw);
      await fetchEntriesForPath(sourceTreeNodeData.collectionId, sourceTreeNodeData.parentNode.path.raw);

      return;
    },
    [batchUpdateCollectionEntry, fetchEntriesForPath]
  );

  //To another collection
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
            return createEntryKind(
              entry.name,
              locationTreeRootNodeData.node.requests.path.raw,
              entry.kind === "Dir",
              entry.class,
              newOrder
            );
          } else {
            return createEntryKind(entry.name, newEntryPath, entry.kind === "Dir", entry.class, entry.order!);
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

      const dropParentEntriesToUpdate = dropParentNodesWithNewOrders.slice(dropOrder + 1).map((entry) => {
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
            },
          };
        }
      });

      const entriesAfterDeletedNodesWithUpdatedOrders = sourceTreeNodeData.parentNode.childNodes
        .filter((entry) => entry.order! > sourceTreeNodeData.node.order!)
        .map((entry) => {
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
            return createEntryKind(
              entry.name,
              locationTreeNodeData.parentNode.path.raw,
              entry.kind === "Dir",
              entry.class,
              newOrder
            );
          } else {
            const newEntryPath = await join(locationTreeNodeData.parentNode.path.raw, entry.path.raw);
            return createEntryKind(entry.name, newEntryPath, entry.kind === "Dir", entry.class, entry.order!);
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

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceTreeNode(source);
      },
      onDrop: async ({ location, source }) => {
        const sourceTreeNodeData = getSourceTreeNodeData(source);
        const locationTreeNodeData = getLocationTreeNodeData(location);
        const locationTreeRootNodeData = getLocationTreeRootNodeData(location);

        const operation = getInstructionFromLocation(location)?.operation;

        if (!sourceTreeNodeData) {
          console.warn("can't drop: no source");
          return;
        }

        if (locationTreeRootNodeData && operation === "combine") {
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

        if (sourceTreeNodeData.collectionId === locationTreeNodeData.collectionId) {
          if (operation === "combine") {
            await handleCombineWithinCollection(sourceTreeNodeData, locationTreeNodeData);
          } else {
            await handleReorderWithinCollection(sourceTreeNodeData, locationTreeNodeData, operation);
          }
        } else {
          await handleMoveToAnotherCollection(sourceTreeNodeData, locationTreeNodeData, operation);
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
  ]);
};
