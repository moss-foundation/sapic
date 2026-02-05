import { environmentSummariesCollection } from "@/db/environmentsSummaries/environmentSummaries";
import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";
import { JsonValue } from "@repo/moss-bindingutils";

import { EnvironmentItemState } from "./types";

const SHARED_STORAGE_ENVIRONMENT_ITEM_STATE_KEY = "workbench.environmentItemState" as const;

interface IEnvironmentItemStateService {
  get: (environmentItemId: string, workspaceId: string) => Promise<EnvironmentItemState>;
  put: (environmentItemState: EnvironmentItemState, workspaceId: string) => Promise<void>;
  remove: (environmentItemId: string, workspaceId: string) => Promise<void>;

  batchGet: (environmentItemIds: string[], workspaceId: string) => Promise<EnvironmentItemState[]>;
  batchPut: (environmentItemStates: EnvironmentItemState[], workspaceId: string) => Promise<void>;
  batchRemove: (environmentItemIds: string[], workspaceId: string) => Promise<void>;
}

export const environmentItemStateService: IEnvironmentItemStateService = {
  get: async (environmentItemId: string, workspaceId: string) => {
    const { value: output } = await sharedStorageIpc.getItem(
      constructEnvironmentItemStateKey(environmentItemId, workspaceId),
      { workspace: workspaceId }
    );

    if (output !== "none") {
      return output.value as unknown as EnvironmentItemState;
    }

    return { id: environmentItemId, order: 0 } satisfies EnvironmentItemState;
  },
  put: async (environmentItemState: EnvironmentItemState, workspaceId: string) => {
    const { id, ...state } = environmentItemState;
    await sharedStorageIpc.putItem(constructEnvironmentItemStateKey(id, workspaceId), state, {
      workspace: workspaceId,
    });

    if (environmentSummariesCollection.has(environmentItemState.id)) {
      environmentSummariesCollection.update(environmentItemState.id, (draft) => {
        draft.order = environmentItemState.order;
      });
    }
  },
  remove: async (environmentItemId: string, workspaceId: string) => {
    await sharedStorageIpc.removeItem(constructEnvironmentItemStateKey(environmentItemId, workspaceId), {
      workspace: workspaceId,
    });
  },
  batchGet: async (environmentItemIds: string[], workspaceId: string) => {
    const keys = environmentItemIds.map((id) => constructEnvironmentItemStateKey(id, workspaceId));
    console.log("batchGet environment item states", { keys, workspaceId });
    const { items: output } = await sharedStorageIpc.batchGetItem(keys, {
      workspace: workspaceId,
    });

    if (!output) return [];

    return environmentItemIds.map((environmentItemId) => {
      const key = constructEnvironmentItemStateKey(environmentItemId, workspaceId);
      const itemValue = output[key];
      if (itemValue !== null && itemValue !== undefined) {
        return { id: environmentItemId, ...(itemValue as Omit<EnvironmentItemState, "id">) };
      }
      return { id: environmentItemId, order: 0 } satisfies EnvironmentItemState;
    });
  },
  batchPut: async (environmentItemStates: EnvironmentItemState[], workspaceId: string) => {
    const items = environmentItemStates.map((environmentItemState) => ({
      key: constructEnvironmentItemStateKey(environmentItemState.id, workspaceId),
      value: { order: environmentItemState.order },
      scope: { workspace: workspaceId },
    }));

    const scope = { workspace: workspaceId };

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

    environmentItemStates.forEach((environmentItemState) => {
      if (!environmentSummariesCollection.has(environmentItemState.id)) return;

      environmentSummariesCollection.update(environmentItemState.id, (draft) => {
        draft.order = environmentItemState.order;
      });
    });
  },
  batchRemove: async (environmentItemIds: string[], workspaceId: string) => {
    const keys = environmentItemIds.map((id) => constructEnvironmentItemStateKey(id, workspaceId));
    await sharedStorageIpc.batchRemoveItem(keys, { workspace: workspaceId });
  },
};

const constructEnvironmentItemStateKey = (environmentItemId: string, workspaceId: string) => {
  return `${SHARED_STORAGE_ENVIRONMENT_ITEM_STATE_KEY}.${workspaceId}.${environmentItemId}`;
};
