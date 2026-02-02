import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";
import { JsonValue } from "@repo/moss-bindingutils";
import { defaultStates } from "./defaults";
import { ActivityBarItemState } from "./types";

const SHARED_STORAGE_ACTIVITY_BAR_ITEM_STATE_KEY = "workbench.activityBarItemState" as const;

export interface IActivityBarItemStateService {
  get: (activityBarId: string, workspaceId: string) => Promise<ActivityBarItemState>;
  put: (activityBarState: ActivityBarItemState, workspaceId: string) => Promise<void>;
  remove: (activityBarId: string, workspaceId: string) => Promise<void>;

  batchGet: (activityBarIds: string[], workspaceId: string) => Promise<ActivityBarItemState[]>;
  batchPut: (activityBarStates: ActivityBarItemState[], workspaceId: string) => Promise<void>;
  batchRemove: (activityBarIds: string[], workspaceId: string) => Promise<void>;
}

export const activityBarItemStateService: IActivityBarItemStateService = {
  get: async (activityBarId: string, workspaceId: string) => {
    const { value: output } = await sharedStorageIpc.getItem(
      constructActivityBarItemStateKey(activityBarId, workspaceId),
      { workspace: workspaceId ?? "application" }
    );

    if (output === "none") {
      return defaultStates.find((state) => state.id === activityBarId)!;
    }

    return output.value as unknown as ActivityBarItemState;
  },
  put: async (activityBarState: ActivityBarItemState, workspaceId: string) => {
    const { id, ...state } = activityBarState;
    await sharedStorageIpc.putItem(constructActivityBarItemStateKey(id, workspaceId), state, {
      workspace: workspaceId ?? "application",
    });
  },
  remove: async (activityBarId: string, workspaceId: string) => {
    await sharedStorageIpc.removeItem(constructActivityBarItemStateKey(activityBarId, workspaceId), {
      workspace: workspaceId ?? "application",
    });
  },

  batchGet: async (activityBarIds: string[], workspaceId: string) => {
    const keys = activityBarIds.map((id) => constructActivityBarItemStateKey(id, workspaceId));
    const { items: output } = await sharedStorageIpc.batchGetItem(keys, {
      workspace: workspaceId ?? "application",
    });

    if (!output) return [];

    return activityBarIds.map((activityBarId): ActivityBarItemState => {
      const key = constructActivityBarItemStateKey(activityBarId, workspaceId);
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
  batchPut: async (activityBarStates: ActivityBarItemState[], workspaceId: string) => {
    const items = activityBarStates.map((activityBarState) => ({
      key: constructActivityBarItemStateKey(activityBarState.id, workspaceId),
      value: {
        order: activityBarState.order,
      },
      scope: { workspace: workspaceId ?? "application" },
    }));

    const scope = { workspace: workspaceId ?? "application" };

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
  batchRemove: async (activityBarIds: string[], workspaceId: string) => {
    await sharedStorageIpc.batchRemoveItem(activityBarIds, {
      workspace: workspaceId ?? "application",
    });
  },
};

const constructActivityBarItemStateKey = (activityBarId: string, workspaceId: string) => {
  return `${SHARED_STORAGE_ACTIVITY_BAR_ITEM_STATE_KEY}.${workspaceId}.${activityBarId}`;
};
