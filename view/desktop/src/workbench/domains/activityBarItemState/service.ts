import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";
import { JsonValue } from "@repo/moss-bindingutils";

import { defaultStates } from "./defaults";
import { ActivityBarItemState } from "./types";

const SHARED_STORAGE_ACTIVITY_BAR_ITEM_STATE_KEY = "workbench.activityBarItemState" as const;

export interface IActivityBarItemStateService {
  get: (activityBarId: string) => Promise<ActivityBarItemState>;
  put: (activityBarState: ActivityBarItemState) => Promise<void>;
  remove: (activityBarId: string) => Promise<void>;

  batchGet: (activityBarIds: string[]) => Promise<ActivityBarItemState[]>;
  batchPut: (activityBarStates: ActivityBarItemState[]) => Promise<void>;
  batchRemove: (activityBarIds: string[]) => Promise<void>;
}

export const activityBarItemStateService: IActivityBarItemStateService = {
  get: async (activityBarId: string) => {
    const { value: output } = await sharedStorageIpc.getItem(
      constructActivityBarItemStateKey(activityBarId),
      "application"
    );

    if (output === "none") {
      return defaultStates.find((state) => state.id === activityBarId)!;
    }

    return output.value as unknown as ActivityBarItemState;
  },
  put: async (activityBarState: ActivityBarItemState) => {
    const { id, ...state } = activityBarState;
    await sharedStorageIpc.putItem(constructActivityBarItemStateKey(id), state, "application");
  },
  remove: async (activityBarId: string) => {
    await sharedStorageIpc.removeItem(constructActivityBarItemStateKey(activityBarId), "application");
  },

  batchGet: async (activityBarIds: string[]) => {
    const keys = activityBarIds.map((id) => constructActivityBarItemStateKey(id));
    const { items: output } = await sharedStorageIpc.batchGetItem(keys, "application");

    if (!output) return [];

    return activityBarIds.map((activityBarId): ActivityBarItemState => {
      const key = constructActivityBarItemStateKey(activityBarId);
      const itemValue = output[key];

      if (itemValue) {
        return { id: activityBarId, ...(itemValue as Omit<ActivityBarItemState, "id">) };
      }

      return {
        id: activityBarId,
        order: 0,
      };
    });
  },
  batchPut: async (activityBarStates: ActivityBarItemState[]) => {
    const items = activityBarStates.map((activityBarState) => ({
      key: constructActivityBarItemStateKey(activityBarState.id),
      value: {
        order: activityBarState.order,
      },
      scope: "application",
    }));

    const scope = "application";

    await sharedStorageIpc.batchPutItem(
      items.reduce(
        (acc, item) => {
          acc[item.key] = item.value;
          return acc;
        },
        {} as Record<string, JsonValue>
      ),
      scope
    );
  },
  batchRemove: async (activityBarIds: string[]) => {
    await sharedStorageIpc.batchRemoveItem(activityBarIds, "application");
  },
};

const constructActivityBarItemStateKey = (activityBarId: string) => {
  return `${SHARED_STORAGE_ACTIVITY_BAR_ITEM_STATE_KEY}.${activityBarId}`;
};
