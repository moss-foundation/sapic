import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";

import { defaultStates } from "./defaults";
import { ActivityBarItemState } from "./types";

const SCOPE = "application" as const;

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
    const { value: output } = await sharedStorageIpc.getItem(`${activityBarId}.order`, SCOPE);
    if (output === "none") {
      return defaultStates.find((state) => state.id === activityBarId)!;
    }
    return { id: activityBarId, order: output.value as number };
  },
  put: async (activityBarState) => {
    await sharedStorageIpc.putItem(`${activityBarState.id}.order`, activityBarState.order, SCOPE);
  },
  remove: async (activityBarId) => {
    await sharedStorageIpc.removeItem(`${activityBarId}.order`, SCOPE);
  },

  batchGet: async (activityBarIds) => {
    const orderKeys = activityBarIds.map((id) => `${id}.order`);
    const { items: output } = await sharedStorageIpc.batchGetItem(orderKeys, SCOPE);
    if (!output) return [];

    return activityBarIds.map((id) => {
      const value = output[`${id}.order`];
      if (value != null) return { id, order: value as number };
      return defaultStates.find((state) => state.id === id) ?? { id, order: 0 };
    });
  },
  batchPut: async (activityBarStates) => {
    const items = Object.fromEntries(activityBarStates.map((state) => [`${state.id}.order`, state.order]));
    await sharedStorageIpc.batchPutItem(items, SCOPE);
  },
  batchRemove: async (activityBarIds) => {
    const orderKeys = activityBarIds.map((id) => `${id}.order`);
    await sharedStorageIpc.batchRemoveItem(orderKeys, SCOPE);
  },
};
