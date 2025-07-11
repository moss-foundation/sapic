import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import {
  DragLocationHistory,
  DropTargetRecord,
  ElementDragPayload,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import { CreateEntryInput, DirConfigurationModel, EntryInfo, ItemConfigurationModel } from "@repo/moss-collection";

import { DragNode, TreeCollectionNode, TreeCollectionRootNode } from "./types";
import { hasDescendant, hasDirectDescendant } from "./utils";

//TODO order should always be set, it's a temporary solution until backend updates it's type
export const sortByOrder = <T extends { order?: number }>(entries: T[]) => {
  return [...entries].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
};

export const isSourceTreeNode = (source: ElementDragPayload): boolean => {
  return source.data.type === "TreeNode";
};

export const doesLocationHaveTreeNode = (location: DragLocationHistory): boolean => {
  if (location.current.dropTargets.length === 0) return false;
  return location.current.dropTargets[0].data.type === "TreeNode";
};

export const getSourceTreeNodeData = (source: ElementDragPayload): DragNode | null => {
  if (source.data.type === "TreeNode") {
    return source.data.data as DragNode;
  }

  return null;
};

export const getLocationTreeNodeData = (location: DragLocationHistory): DragNode | null => {
  if (location.current.dropTargets[0].data.type === "TreeNode") {
    return location.current.dropTargets[0].data.data as DragNode;
  }

  return null;
};

export const getAllNestedEntries = (node: TreeCollectionNode): EntryInfo[] => {
  const result: EntryInfo[] = [];

  const { childNodes, ...entryInfo } = node;
  result.push(entryInfo);

  for (const child of childNodes) {
    result.push(...getAllNestedEntries(child));
  }

  return result;
};

export const convertEntryInfoToCreateInput = (
  entry: EntryInfo,
  newCollectionPath: string = "requests"
): CreateEntryInput => {
  console.log({ newCollectionPath });
  if (entry.kind === "Dir") {
    return {
      dir: {
        name: entry.name,
        path: newCollectionPath,
        order: entry.order ?? 0,
        configuration: {
          request: {
            http: {},
          },
        },
      },
    };
  } else {
    return {
      item: {
        name: entry.name,
        path: newCollectionPath,
        order: entry.order ?? 0,
        configuration: {
          request: {
            http: {
              requestParts: {
                method: "GET",
              },
            },
          },
        },
      },
    };
  }
};

export const createDirConfiguration = (nodeClass: TreeCollectionNode["class"]): DirConfigurationModel => {
  switch (nodeClass) {
    case "Request":
      return { request: { http: {} } };
    case "Endpoint":
      return { request: { http: {} } };
    case "Component":
      return { component: {} };
    case "Schema":
      return { schema: {} };
    default:
      return { request: { http: {} } };
  }
};

//FIXME: This is a temporary solution until we have a proper configuration model
export const createItemConfiguration = (nodeClass: TreeCollectionNode["class"]): ItemConfigurationModel => {
  switch (nodeClass) {
    case "Request":
      return {
        request: {
          http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
    case "Endpoint":
      return {
        endpoint: {
          Http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
    case "Component":
      return { component: {} };
    case "Schema":
      return { schema: {} };
    default:
      return {
        request: {
          http: {
            requestParts: {
              method: "GET",
            },
          },
        },
      };
  }
};

export const getInstruction = (self: DropTargetRecord): Instruction | null => {
  return extractInstruction(self.data);
};

export const canDropNode = (sourceTarget: DragNode, dropTarget: DragNode, node: TreeCollectionNode) => {
  if (sourceTarget.node.kind !== "Dir") {
    // if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
    //   return false;
    // }
  }

  if (sourceTarget.node.kind === "Dir") {
    // if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
    //   return false;
    // }

    if (hasDirectDescendant(dropTarget.node, node)) {
      return false;
    }

    if (hasDescendant(sourceTarget.node, node)) {
      return false;
    }

    if (sourceTarget.node.id === node.id) {
      return false;
    }
  }

  return true;
};

export const canDropRootNode = (sourceTarget: DragNode, dropTarget: DragNode, node: TreeCollectionRootNode) => {
  if (sourceTarget.node.kind === "Dir") {
    // if (hasDirectSimilarDescendant(node, sourceTarget.node)) {
    //   return false;
    // }

    if (hasDirectDescendant(dropTarget.node, node)) {
      return false;
    }

    if (hasDescendant(sourceTarget.node, node)) {
      return false;
    }

    if (sourceTarget.node.id === node.id) {
      return false;
    }
  }

  return true;
};
