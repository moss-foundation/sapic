import { extractInstruction, Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import {
  DragLocationHistory,
  DropTargetRecord,
  ElementDragPayload,
} from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";
import {
  BatchCreateEntryKind,
  CreateEntryInput,
  DirConfigurationModel,
  EntryInfo,
  ItemConfigurationModel,
} from "@repo/moss-collection";

import { DragNode, DropNode, TreeCollectionNode } from "./types";
import { hasDescendant, hasDirectDescendant } from "./utils";

//TODO order should always be set, it's a temporary solution until backend updates it's type
export const sortByOrder = <T extends { order?: number }>(entries: T[]) => {
  return [...entries].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
};

export const isSourceTreeNode = (source: ElementDragPayload): boolean => {
  return source.data.type === "TreeNode";
};

export const isSourceTreeRootNode = (source: ElementDragPayload): boolean => {
  return source.data.type === "TreeRootNode";
};

export const doesLocationHaveTreeNode = (location: DragLocationHistory): boolean => {
  if (location.current.dropTargets.length === 0) return false;
  return location.current.dropTargets[0].data.type === "TreeNode";
};

export const getSourceTreeNodeData = (source: ElementDragPayload): DragNode => {
  return source.data.data as DragNode;
};

export const getLocationTreeNodeData = (location: DragLocationHistory): DropNode => {
  const instruction = extractInstruction(location.current.dropTargets[0].data);

  return {
    ...(location.current.dropTargets[0].data.data as DragNode),
    "instruction": instruction ?? undefined,
  };
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
      DIR: {
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
      ITEM: {
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

export const getInstructionFromSelf = (self: DropTargetRecord): Instruction | null => {
  return extractInstruction(self.data);
};

export const getInstructionFromLocation = (location: DragLocationHistory): Instruction | null => {
  return extractInstruction(location.current.dropTargets[0].data);
};

export const canDropNode = (sourceTarget: DragNode, dropTarget: DragNode, node: TreeCollectionNode) => {
  if (sourceTarget.node.kind === "Dir") {
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

export const createEntryKind = (
  name: string,
  path: string,
  isAddingFolder: boolean,
  entryClass: EntryInfo["class"],
  order: number
): BatchCreateEntryKind => {
  if (isAddingFolder) {
    return {
      DIR: {
        name,
        path,
        order,
        configuration: createDirConfiguration(entryClass),
      },
    };
  }

  return {
    ITEM: {
      name,
      path,
      order,
      configuration: createItemConfiguration(entryClass),
    },
  };
};

//FIXME: This is a temporary solution until we have a proper configuration model
export const createDirConfiguration = (entryClass: EntryInfo["class"]): DirConfigurationModel => {
  switch (entryClass) {
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
export const createItemConfiguration = (entryClass: EntryInfo["class"]): ItemConfigurationModel => {
  switch (entryClass) {
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

export const normalizeEntry = (entry: CreateEntryInput) => {
  if ("ITEM" in entry) return entry.ITEM;
  if ("DIR" in entry) return entry.DIR;
  return entry;
};
