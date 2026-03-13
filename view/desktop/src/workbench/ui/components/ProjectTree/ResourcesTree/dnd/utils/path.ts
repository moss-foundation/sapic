import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ListProjectResourceItem } from "@repo/ipc";
import { BatchUpdateResourceKind } from "@repo/moss-project";
import { join } from "@tauri-apps/api/path";

import { ResourcesTreeRoot } from "../../../TreeRoot/types";
import { ResourceNode } from "../../types";

const getPathWithoutName = async (
  node: ResourceNode | ListProjectResourceItem
): Promise<ListProjectResourceItem["path"]> => {
  const newSegments = node.path.segments.filter((segment) => segment !== node.name);
  const newRaw = newSegments.length > 0 ? await join(...newSegments) : "";

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

const removePathBeforeName = async (path: ListProjectResourceItem["path"], name: string) => {
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

export const prepareNestedDirResourcesForDrop = async (
  resources: ListProjectResourceItem[]
): Promise<ListProjectResourceItem[]> => {
  const rootResourceName = resources[0].name;

  const resourcesPreparedForDrop: ListProjectResourceItem[] = [];

  for await (const resource of resources) {
    const newResourcePath = await removePathBeforeName(resource.path, rootResourceName);

    resourcesPreparedForDrop.push({
      ...resource,
      path: newResourcePath,
    });
  }

  const resourcesWithoutName = await Promise.all(
    resourcesPreparedForDrop.map(async (resource) => {
      const pathWithoutName = await getPathWithoutName(resource);

      return {
        ...resource,
        path: pathWithoutName,
      };
    })
  );

  return resourcesWithoutName;
};

export const prepareResourcesForCreation = async (
  resources: ListProjectResourceItem[]
): Promise<ListProjectResourceItem[]> => {
  const rootResourceName = resources[0].name;

  const resourcesPreparedForDrop: ListProjectResourceItem[] = [];

  for await (const resource of resources) {
    const newResourcePath = await removePathBeforeName(resource.path, rootResourceName);

    resourcesPreparedForDrop.push({
      ...resource,
      path: newResourcePath,
    });
  }

  const resourcesWithoutName = await Promise.all(
    resourcesPreparedForDrop.map(async (resource) => {
      const pathWithoutName = await getPathWithoutName(resource);

      return {
        ...resource,
        path: pathWithoutName,
      };
    })
  );

  return resourcesWithoutName;
};

export const makeItemUpdatePayload = ({
  id,
  order,
  path,
}: {
  id: string;
  order?: number;
  path?: string;
}): BatchUpdateResourceKind => ({
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
}): BatchUpdateResourceKind => ({
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
  nodes: ResourceNode[];
  removedNode: ResourceNode;
}) => {
  const sortedChildren = sortObjectsByOrder(nodes);
  return sortedChildren
    .filter((c) => c.id !== removedNode.id && c.order! > removedNode.order!)
    .map((resource) =>
      resource.kind === "Dir"
        ? makeDirUpdatePayload({ id: resource.id, order: resource.order! - 1 })
        : makeItemUpdatePayload({ id: resource.id, order: resource.order! - 1 })
    );
};

export const reorderedNodesForDifferentDirPayload = ({
  node,
  newNode,
  moveToIndex,
}: {
  node: ResourceNode | ResourcesTreeRoot;
  newNode: ResourceNode;
  moveToIndex: number;
}) => {
  const sortedTargetNodes = sortObjectsByOrder(node.childNodes);

  const targetResourcesToUpdate = [
    ...sortedTargetNodes.slice(0, moveToIndex),
    newNode,
    ...sortedTargetNodes.slice(moveToIndex),
  ]
    .map((resource, index) => ({
      ...resource,
      order: index + 1,
    }))
    .filter((node) => {
      const nodeInLocation = node.childNodes.find((n) => n.id === node.id);
      return nodeInLocation?.order !== node.order;
    })
    .map((resource) => {
      const isAlreadyInLocation = node.childNodes.some((n) => n.id === resource.id);
      const newResourcePath = isAlreadyInLocation ? undefined : "path" in node ? node.path.raw : "";

      if (resource.kind === "Dir") {
        return makeDirUpdatePayload({
          id: resource.id,
          order: resource.order,
          path: newResourcePath,
        });
      } else {
        return makeItemUpdatePayload({
          id: resource.id,
          order: resource.order,
          path: newResourcePath,
        });
      }
    });

  return targetResourcesToUpdate;
};

export const resolveParentPath = (parentNode: ResourceNode | ResourcesTreeRoot): string =>
  "path" in parentNode ? parentNode.path.raw : "";
