import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";

interface BaseOptions {
  edge?: Edge | null;
}

interface SwapByIndexOptions extends BaseOptions {
  mode: "index";
}

interface SwapByIdOptions extends BaseOptions {
  mode: "id";
  idKey?: string;
}

export const swap = <T extends Record<string, unknown>>(
  list: T[],
  from: number | string,
  to: number | string,
  options: SwapByIndexOptions | SwapByIdOptions
): T[] | null => {
  const { mode, edge = null } = options;
  const idKey = options.mode === "id" ? (options.idKey ?? "id") : undefined;

  const updatedItems = [...list];

  let fromIndex: number;
  let toIndex: number;

  if (mode === "index") {
    fromIndex = from as number;
    toIndex = to as number;

    if (fromIndex === toIndex || fromIndex < 0 || toIndex < 0) {
      return updatedItems;
    }
  } else {
    const key = idKey as keyof T;
    fromIndex = updatedItems.findIndex((item) => item[key] === from);
    toIndex = updatedItems.findIndex((item) => item[key] === to);

    if (fromIndex === -1 || toIndex === -1 || fromIndex === toIndex) {
      return null;
    }
  }

  if (!edge) {
    [updatedItems[fromIndex], updatedItems[toIndex]] = [updatedItems[toIndex], updatedItems[fromIndex]];
  } else {
    const [itemToMove] = updatedItems.splice(fromIndex, 1);
    const newToIndex = fromIndex < toIndex ? toIndex - 1 : toIndex;

    if (edge === "top" || edge === "left") {
      updatedItems.splice(newToIndex, 0, itemToMove);
    } else {
      updatedItems.splice(newToIndex + 1, 0, itemToMove);
    }
  }

  updatedItems.forEach((item, index) => {
    if (typeof item === "object" && item !== null && "order" in item) {
      (item as Record<string, unknown>).order = index;
    }
  });

  return updatedItems;
};
