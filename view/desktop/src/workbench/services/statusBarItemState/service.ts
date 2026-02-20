import {
  batchGetItemOrder,
  batchPutItemOrder,
  batchRemoveItemOrder,
  getItemOrder,
  putItemOrder,
  removeItemOrder,
} from "@/workbench/usecases/sharedStorage/itemOrder";

import { defaultStates } from "./defaults";

export interface IStatusBarItemStateService {
  get: (id: string) => Promise<number | undefined>;
  put: (id: string, order: number) => Promise<void>;
  remove: (id: string) => Promise<void>;

  batchGet: (ids: string[]) => Promise<number[]>;
  batchPut: (states: Record<string, number>) => Promise<void>;
  batchRemove: (ids: string[]) => Promise<void>;
}

export const statusBarItemStateService: IStatusBarItemStateService = {
  get: async (id) => {
    const { value } = await getItemOrder(id);
    if (value === "none") {
      return defaultStates[id];
    }
    return value.value as number;
  },
  put: async (id, order) => {
    await putItemOrder(id, order);
  },
  remove: async (id) => {
    await removeItemOrder(id);
  },

  batchGet: async (ids) => {
    const { items: output } = await batchGetItemOrder(ids);
    if (!output) return [];

    return ids.map((id) => {
      const value = output[`${id}.order`];
      return (value != null ? (value as number) : defaultStates[id]) ?? 0;
    });
  },
  batchPut: async (states) => {
    const items = Object.fromEntries(Object.entries(states).map(([id, order]) => [id, order]));
    await batchPutItemOrder(items);
  },
  batchRemove: async (ids) => {
    await batchRemoveItemOrder(ids);
  },
};
