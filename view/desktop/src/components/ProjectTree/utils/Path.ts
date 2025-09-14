import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { BatchUpdateEntryKind, StreamEntriesEvent } from "@repo/moss-project";
import { join } from "@tauri-apps/api/path";

import { ProjectTreeNode, ProjectTreeRootNode } from "../types";

export const getPathWithoutName = async (
  node: ProjectTreeNode | StreamEntriesEvent
): Promise<StreamEntriesEvent["path"]> => {
  const newSegments = node.path.segments.filter((segment) => segment !== node.name);
  const newRaw = newSegments.length > 0 ? await join(...newSegments) : "";

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const getPathWithoutParentPath = async (
  path: StreamEntriesEvent["path"],
  parentPath: StreamEntriesEvent["path"]
): Promise<StreamEntriesEvent["path"]> => {
  const newSegments = path.segments.filter((segment) => !parentPath.segments.includes(segment));
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const removePathBeforeName = async (path: StreamEntriesEvent["path"], name: string) => {
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

export const prepareNestedDirEntriesForDrop = async (entries: StreamEntriesEvent[]): Promise<StreamEntriesEvent[]> => {
  const rootEntryName = entries[0].name;

  const entriesPreparedForDrop: StreamEntriesEvent[] = [];

  for await (const entry of entries) {
    const newEntryPath = await removePathBeforeName(entry.path, rootEntryName);

    entriesPreparedForDrop.push({
      ...entry,
      path: newEntryPath,
    });
  }

  const entriesWithoutName = await Promise.all(
    entriesPreparedForDrop.map(async (entry) => {
      const pathWithoutName = await getPathWithoutName(entry);

      return {
        ...entry,
        path: pathWithoutName,
      };
    })
  );

  return entriesWithoutName;
};

export const prepareEntriesForCreation = async (entries: StreamEntriesEvent[]): Promise<StreamEntriesEvent[]> => {
  const rootEntryName = entries[0].name;

  const entriesPreparedForDrop: StreamEntriesEvent[] = [];

  for await (const entry of entries) {
    const newEntryPath = await removePathBeforeName(entry.path, rootEntryName);

    entriesPreparedForDrop.push({
      ...entry,
      path: newEntryPath,
    });
  }

  const entriesWithoutName = await Promise.all(
    entriesPreparedForDrop.map(async (entry) => {
      const pathWithoutName = await getPathWithoutName(entry);

      return {
        ...entry,
        path: pathWithoutName,
      };
    })
  );

  return entriesWithoutName;
};

export const makeItemUpdatePayload = ({
  id,
  order,
  path,
}: {
  id: string;
  order?: number;
  path?: string;
}): BatchUpdateEntryKind => ({
  ITEM: {
    id,
    ...(order !== undefined ? { order } : {}),
    ...(path !== undefined ? { path } : {}),
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
});

export const makeDirUpdatePayload = ({
  id,
  order,
  path,
}: {
  id: string;
  order?: number;
  path?: string;
}): BatchUpdateEntryKind => ({
  DIR: {
    id,
    ...(order !== undefined ? { order } : {}),
    ...(path !== undefined ? { path } : {}),
  },
});

export const siblingsAfterRemovalPayload = ({
  nodes,
  removedNode,
}: {
  nodes: ProjectTreeNode[];
  removedNode: ProjectTreeNode;
}) => {
  const sortedChildren = sortObjectsByOrder(nodes);
  return sortedChildren
    .filter((c) => c.id !== removedNode.id && c.order! > removedNode.order!)
    .map((entry) =>
      entry.kind === "Dir"
        ? makeDirUpdatePayload({ id: entry.id, order: entry.order! - 1 })
        : makeItemUpdatePayload({ id: entry.id, order: entry.order! - 1 })
    );
};

export const reorderedNodesForSameDirPayload = ({
  nodes,
  movedId,
  moveToIndex,
}: {
  nodes: ProjectTreeNode[];
  movedId: string;
  moveToIndex: number;
}) => {
  const nodeToMove = nodes.find((n) => n.id === movedId);

  if (!nodeToMove) {
    console.error("Node to move not found", { movedId, nodes });
    return [];
  }

  const sortedParentNodes = sortObjectsByOrder(nodes);
  const updatedSourceNodesPayload = [
    ...sortedParentNodes.slice(0, moveToIndex).filter((entry) => entry.id !== nodeToMove.id),
    nodeToMove,
    ...sortedParentNodes.slice(moveToIndex).filter((entry) => entry.id !== nodeToMove.id),
  ]
    .map((entry, index) => ({
      ...entry,
      order: index + 1,
    }))
    .filter((entry) => {
      const nodeInLocation = nodes.find((n) => n.id === entry.id);
      return nodeInLocation?.order !== entry.order;
    })
    .map((entry) => {
      if (entry.kind === "Dir") {
        return makeDirUpdatePayload({ id: entry.id, order: entry.order });
      } else {
        return makeItemUpdatePayload({ id: entry.id, order: entry.order });
      }
    });

  return updatedSourceNodesPayload;
};

export const reorderedNodesForDifferentDirPayload = ({
  node,
  newNode,
  moveToIndex,
}: {
  node: ProjectTreeNode | ProjectTreeRootNode;
  newNode: ProjectTreeNode;
  moveToIndex: number;
}) => {
  const sortedTargetNodes = sortObjectsByOrder(node.childNodes);

  const targetEntriesToUpdate = [
    ...sortedTargetNodes.slice(0, moveToIndex),
    newNode,
    ...sortedTargetNodes.slice(moveToIndex),
  ]
    .map((entry, index) => ({
      ...entry,
      order: index + 1,
    }))
    .filter((node) => {
      const nodeInLocation = node.childNodes.find((n) => n.id === node.id);
      return nodeInLocation?.order !== node.order;
    })
    .map((entry) => {
      const isAlreadyInLocation = node.childNodes.some((n) => n.id === entry.id);
      const newEntryPath = isAlreadyInLocation ? undefined : "path" in node ? node.path.raw : "";

      if (entry.kind === "Dir") {
        return makeDirUpdatePayload({
          id: entry.id,
          order: entry.order,
          path: newEntryPath,
        });
      } else {
        return makeItemUpdatePayload({
          id: entry.id,
          order: entry.order,
          path: newEntryPath,
        });
      }
    });

  return targetEntriesToUpdate;
};

export const resolveParentPath = (parentNode: ProjectTreeNode | ProjectTreeRootNode): string =>
  "path" in parentNode ? parentNode.path.raw : "";
