import { environmentSummariesCollection } from "@/db/environmentsSummaries/environmentSummaries";
import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";
import { JsonValue } from "@repo/moss-bindingutils";

import { EnvironmentListItemState } from "./types";

const SHARED_STORAGE_ENVIRONMENT_LIST_ITEM_STATE_KEY = "workbench.environmentListItemState" as const;

interface IEnvironmentListItemStateService {
  get: (environmentListItemId: string, workspaceId: string) => Promise<EnvironmentListItemState>;
  put: (environmentListItemState: EnvironmentListItemState, workspaceId: string) => Promise<void>;
  remove: (environmentItemId: string, workspaceId: string) => Promise<void>;

  batchGet: (environmentListItemIds: string[], workspaceId: string) => Promise<EnvironmentListItemState[]>;
  batchPut: (environmentListItemStates: EnvironmentListItemState[], workspaceId: string) => Promise<void>;
  batchRemove: (environmentItemIds: string[], workspaceId: string) => Promise<void>;
}

export const environmentListItemStateService: IEnvironmentListItemStateService = {
  get: async (environmentListItemId: string, workspaceId: string) => {
    const { value: output } = await sharedStorageIpc.getItem(
      constructEnvironmentListItemStateKey(environmentListItemId, workspaceId),
      { workspace: workspaceId }
    );

    if (output !== "none") {
      return output.value as unknown as EnvironmentListItemState;
    }

    return { id: environmentListItemId, expanded: false } satisfies EnvironmentListItemState;
  },
  put: async (environmentListItemState: EnvironmentListItemState, workspaceId: string) => {
    const { id, ...state } = environmentListItemState;
    await sharedStorageIpc.putItem(constructEnvironmentListItemStateKey(id, workspaceId), state, {
      workspace: workspaceId,
    });

    if (environmentSummariesCollection.has(environmentListItemState.id)) {
      environmentSummariesCollection.update(environmentListItemState.id, (draft) => {
        draft.expanded = environmentListItemState.expanded;
      });
    }
  },
  remove: async (environmentListItemId: string, workspaceId: string) => {
    await sharedStorageIpc.removeItem(constructEnvironmentListItemStateKey(environmentListItemId, workspaceId), {
      workspace: workspaceId,
    });
  },
  batchGet: async (environmentListItemIds: string[], workspaceId: string) => {
    const keys = environmentListItemIds.map((id) => constructEnvironmentListItemStateKey(id, workspaceId));
    const { items: output } = await sharedStorageIpc.batchGetItem(keys, {
      workspace: workspaceId,
    });

    if (!output) return [];

    return environmentListItemIds.map((environmentListItemId) => {
      const key = constructEnvironmentListItemStateKey(environmentListItemId, workspaceId);
      const itemValue = output[key];
      if (itemValue !== null && itemValue !== undefined) {
        return { id: environmentListItemId, ...(itemValue as Omit<EnvironmentListItemState, "id">) };
      }
      return { id: environmentListItemId, expanded: false } satisfies EnvironmentListItemState;
    });
  },
  batchPut: async (environmentListItemStates: EnvironmentListItemState[], workspaceId: string) => {
    const items = environmentListItemStates.map((environmentListItemState) => ({
      key: constructEnvironmentListItemStateKey(environmentListItemState.id, workspaceId),
      value: { expanded: environmentListItemState.expanded },
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

    environmentListItemStates.forEach((environmentListItemState) => {
      if (!environmentSummariesCollection.has(environmentListItemState.id)) return;

      environmentSummariesCollection.update(environmentListItemState.id, (draft) => {
        draft.expanded = environmentListItemState.expanded;
      });
    });
  },
  batchRemove: async (environmentListItemIds: string[], workspaceId: string) => {
    const keys = environmentListItemIds.map((id) => constructEnvironmentListItemStateKey(id, workspaceId));
    await sharedStorageIpc.batchRemoveItem(keys, { workspace: workspaceId });
  },
};

const constructEnvironmentListItemStateKey = (environmentListItemId: string, workspaceId: string) => {
  return `${SHARED_STORAGE_ENVIRONMENT_LIST_ITEM_STATE_KEY}.${workspaceId}.${environmentListItemId}`;
};
