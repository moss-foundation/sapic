import {
  batchGetItemOrder,
  batchPutItemOrder,
  batchRemoveItemOrder,
  getItemOrder,
  putItemOrder,
  removeItemOrder,
} from "@/workbench/usecases/sharedStorage/itemOrder";

import { defaultStates } from "./defaults";
import { ActivityBarItemState } from "./types";

const SCOPE = "" as const;

export interface IActivityBarItemStateService {
  get: (activityBarId: string) => Promise<ActivityBarItemState>;
  put: (activityBarState: ActivityBarItemState) => Promise<void>;
  remove: (activityBarId: string) => Promise<void>;

  batchGet: (activityBarIds: string[]) => Promise<ActivityBarItemState[]>;
  batchPut: (activityBarStates: ActivityBarItemState[]) => Promise<void>;
  batchRemove: (activityBarIds: string[]) => Promise<void>;
}

export const activityBarItemStateService: IActivityBarItemStateService = {
  get: async (activityBarId) => {
    const { value } = await getItemOrder(activityBarId, SCOPE);
    if (value === "none") {
      return defaultStates.find((state) => state.id === activityBarId)!;
    }
    return { id: activityBarId, order: value.value as number };
  },
  put: async (activityBarState) => {
    await putItemOrder(activityBarState.id, activityBarState.order, SCOPE);
  },
  remove: async (activityBarId) => {
    await removeItemOrder(activityBarId, SCOPE);
  },

  batchGet: async (activityBarIds) => {
    const { items: output } = await batchGetItemOrder(activityBarIds, SCOPE);
    if (!output) return [];

    return activityBarIds.map((id) => {
      const value = output[`${id}.order`];
      if (value != null) return { id, order: value as number };
      return defaultStates.find((state) => state.id === id) ?? { id, order: 0 };
    });
  },
  batchPut: async (activityBarStates) => {
    const items = Object.fromEntries(activityBarStates.map((state) => [state.id, state.order]));
    await batchPutItemOrder(items, SCOPE);
  },
  batchRemove: async (activityBarIds) => {
    await batchRemoveItemOrder(activityBarIds, SCOPE);
  },
};
