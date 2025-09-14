import { useCallback, useEffect } from "react";

import { useDeleteProjectEntry } from "@/hooks";
import { useFetchEntriesForPath } from "@/hooks/project/derivedHooks/useFetchEntriesForPath";
import { useBatchCreateProjectEntry } from "@/hooks/project/useBatchCreateProjectEntry";
import { useBatchUpdateProjectEntry } from "@/hooks/project/useBatchUpdateProjectEntry";
import { useCreateProjectEntry } from "@/hooks/project/useCreateProjectEntry";
import { useUpdateProjectEntry } from "@/hooks/project/useUpdateProjectEntry";
import { Operation } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

import { DragNode, DropNode, DropRootNode } from "../types";
import {
  canDropNode,
  createEntryKind,
  getAllNestedEntries,
  getInstructionFromLocation,
  getLocationProjectTreeNodeData,
  getLocationProjectTreeRootNodeData,
  getSourceProjectTreeNodeData,
  hasDirectDescendantWithSimilarName,
  isSourceProjectTreeNode,
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
  const { mutateAsync: createProjectEntry } = useCreateProjectEntry();
  const { mutateAsync: updateProjectEntry } = useUpdateProjectEntry();
  const { mutateAsync: deleteProjectEntry } = useDeleteProjectEntry();

  const { mutateAsync: batchCreateProjectEntry } = useBatchCreateProjectEntry();
  const { mutateAsync: batchUpdateProjectEntry } = useBatchUpdateProjectEntry();

  const { fetchEntriesForPath } = useFetchEntriesForPath();

  //Within Project
  const handleCombineWithinProject = useCallback(
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
      await batchUpdateProjectEntry({
        projectId: sourceTreeNodeData.projectId,
        entries: {
          entries: allUpdates,
        },
      });

      await fetchEntriesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));

      await fetchEntriesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [batchUpdateProjectEntry, fetchEntriesForPath]
  );

  const handleReorderWithinProject = useCallback(
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

        await batchUpdateProjectEntry({
          projectId: sourceTreeNodeData.projectId,
          entries: {
            entries: updatedSourceNodesPayload,
          },
        });

        await fetchEntriesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

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

      await batchUpdateProjectEntry({
        projectId: sourceTreeNodeData.projectId,
        entries: {
          entries: allEntriesToUpdate,
        },
      });

      await fetchEntriesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));
      await fetchEntriesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [batchUpdateProjectEntry, fetchEntriesForPath]
  );

  //To Another Project
  const handleMoveToAnotherProject = useCallback(
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

      await batchUpdateProjectEntry({
        projectId: sourceTreeNodeData.projectId,
        entries: {
          entries: updatedSourceEntriesPayload,
        },
      });

      await batchUpdateProjectEntry({
        projectId: locationTreeNodeData.projectId,
        entries: {
          entries: targetEntriesToUpdate,
        },
      });

      await deleteProjectEntry({
        projectId: sourceTreeNodeData.projectId,
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

      await batchCreateProjectEntry({
        projectId: locationTreeNodeData.projectId,
        input: {
          entries: batchCreateEntryInput,
        },
      });

      await fetchEntriesForPath(
        locationTreeNodeData.projectId,
        "path" in locationTreeNodeData.parentNode ? locationTreeNodeData.parentNode.path.raw : ""
      );
      await fetchEntriesForPath(
        sourceTreeNodeData.projectId,
        "path" in sourceTreeNodeData.parentNode ? sourceTreeNodeData.parentNode.path.raw : ""
      );
    },
    [batchUpdateProjectEntry, deleteProjectEntry, batchCreateProjectEntry, fetchEntriesForPath]
  );

  const handleCombineToAnotherProject = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeNodeData: DropNode) => {
      const allEntries = getAllNestedEntries(sourceTreeNodeData.node);
      const entriesPreparedForCreation = await prepareEntriesForCreation(allEntries);

      const newOrder = locationTreeNodeData.node.childNodes.length + 1;

      await deleteProjectEntry({
        projectId: sourceTreeNodeData.projectId,
        input: { id: sourceTreeNodeData.node.id },
      });

      const updatedSourceEntriesPayload = siblingsAfterRemovalPayload({
        nodes: sourceTreeNodeData.parentNode.childNodes,
        removedNode: sourceTreeNodeData.node,
      });

      await batchUpdateProjectEntry({
        projectId: sourceTreeNodeData.projectId,
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

      await batchCreateProjectEntry({
        projectId: locationTreeNodeData.projectId,
        input: { entries: batchCreateEntryInput },
      });

      await fetchEntriesForPath(locationTreeNodeData.projectId, resolveParentPath(locationTreeNodeData.parentNode));
      await fetchEntriesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));

      return;
    },
    [batchCreateProjectEntry, batchUpdateProjectEntry, deleteProjectEntry, fetchEntriesForPath]
  );

  //To Another Project's Root
  const handleCombineToAnotherProjectRoot = useCallback(
    async (sourceTreeNodeData: DragNode, locationTreeRootNodeData: DropRootNode) => {
      const allEntries = getAllNestedEntries(sourceTreeNodeData.node);
      const entriesWithoutName = await prepareNestedDirEntriesForDrop(allEntries);

      const newOrder = locationTreeRootNodeData.node.childNodes.length + 1;

      await deleteProjectEntry({
        projectId: sourceTreeNodeData.projectId,
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

      await batchUpdateProjectEntry({
        projectId: sourceTreeNodeData.projectId,
        entries: {
          entries: updatedSourceEntriesPayload,
        },
      });

      await batchCreateProjectEntry({
        projectId: locationTreeRootNodeData.projectId,
        input: {
          entries: batchCreateEntryInput,
        },
      });

      await fetchEntriesForPath(locationTreeRootNodeData.projectId, "");
      await fetchEntriesForPath(sourceTreeNodeData.projectId, resolveParentPath(sourceTreeNodeData.parentNode));
    },
    [deleteProjectEntry, batchUpdateProjectEntry, batchCreateProjectEntry, fetchEntriesForPath]
  );

  useEffect(() => {
    return monitorForElements({
      canMonitor({ source }) {
        return isSourceProjectTreeNode(source);
      },
      onDrop: async ({ location, source }) => {
        const sourceTreeNodeData = getSourceProjectTreeNodeData(source);
        const locationTreeNodeData = getLocationProjectTreeNodeData(location);
        const locationTreeRootNodeData = getLocationProjectTreeRootNodeData(location);

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
          if (hasDirectDescendantWithSimilarName(locationTreeRootNodeData.node, sourceTreeNodeData.node)) {
            console.warn("can't drop: has direct similar descendant");
            return;
          }

          await handleCombineToAnotherProjectRoot(sourceTreeNodeData, locationTreeRootNodeData);
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

        const isSameProject = sourceTreeNodeData.projectId === locationTreeNodeData.projectId;
        if (isSameProject) {
          if (operation === "combine") {
            await handleCombineWithinProject(sourceTreeNodeData, locationTreeNodeData);
          } else {
            await handleReorderWithinProject(sourceTreeNodeData, locationTreeNodeData, operation);
          }
        } else {
          if (operation === "combine") {
            await handleCombineToAnotherProject(sourceTreeNodeData, locationTreeNodeData);
          } else {
            await handleMoveToAnotherProject(sourceTreeNodeData, locationTreeNodeData, operation);
          }
        }
      },
    });
  }, [
    batchCreateProjectEntry,
    batchUpdateProjectEntry,
    createProjectEntry,
    deleteProjectEntry,
    fetchEntriesForPath,
    handleCombineWithinProject,
    handleMoveToAnotherProject,
    handleReorderWithinProject,
    handleCombineToAnotherProjectRoot,
    updateProjectEntry,
    handleCombineToAnotherProject,
  ]);
};
