import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";

import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import { JsonValue } from "@repo/moss-bindingutils";
import { TreeItemState } from "./types";

const SHARED_STORAGE_TREE_ITEM_STATE_KEY = "workbench.treeItemState" as const;

interface ITreeItemStateService {
  get: (treeItemId: string, workspaceId: string) => Promise<TreeItemState>;
  put: (treeItemState: TreeItemState, workspaceId: string) => Promise<void>;
  remove: (treeItemId: string, workspaceId: string) => Promise<void>;

  batchGet: (treeItemIds: string[], workspaceId: string) => Promise<TreeItemState[]>;
  batchPut: (treeItemStates: TreeItemState[], workspaceId: string) => Promise<void>;
  batchRemove: (treeItemIds: string[], workspaceId: string) => Promise<void>;
}

export const treeItemStateService: ITreeItemStateService = {
  get: async (treeItemId: string, workspaceId: string) => {
    const { value: output } = await sharedStorageIpc.getItem(constructTreeItemStateKey(treeItemId, workspaceId), {
      workspace: workspaceId ?? "application",
    });

    if (output !== "none") {
      return output.value as unknown as TreeItemState;
    }

    return { id: treeItemId, expanded: false, order: 0 };
  },
  put: async (treeItemState: TreeItemState, workspaceId: string) => {
    const { id, ...state } = treeItemState;
    await sharedStorageIpc.putItem(constructTreeItemStateKey(id, workspaceId), state, {
      workspace: workspaceId ?? "application",
    });

    if (projectSummariesCollection.has(treeItemState.id)) {
      projectSummariesCollection.update(treeItemState.id, (draft) => {
        draft.order = treeItemState.order;
        draft.expanded = treeItemState.expanded;
      });
    }
  },
  remove: async (treeItemId: string, workspaceId: string) => {
    await sharedStorageIpc.removeItem(constructTreeItemStateKey(treeItemId, workspaceId), {
      workspace: workspaceId ?? "application",
    });
  },
  batchGet: async (treeItemIds: string[], workspaceId: string) => {
    const keys = treeItemIds.map((id) => constructTreeItemStateKey(id, workspaceId));
    const { items: output } = await sharedStorageIpc.batchGetItem(keys, {
      workspace: workspaceId ?? "application",
    });

    if (!output) return [];

    return treeItemIds.map((treeItemId) => {
      const key = constructTreeItemStateKey(treeItemId, workspaceId);
      const itemValue = output[key];
      if (itemValue !== null && itemValue !== undefined) {
        return { id: treeItemId, ...(itemValue as Omit<TreeItemState, "id">) };
      }
      return { id: treeItemId, expanded: false, order: 0 };
    });
  },
  batchPut: async (treeItemStates: TreeItemState[], workspaceId: string) => {
    const items = treeItemStates.map((treeItemState) => ({
      key: constructTreeItemStateKey(treeItemState.id, workspaceId),
      value: {
        expanded: treeItemState.expanded,
        order: treeItemState.order,
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

    console.log("treeItemStates", treeItemStates);
    treeItemStates.forEach((treeItemState) => {
      if (!projectSummariesCollection.has(treeItemState.id)) return;

      projectSummariesCollection.update(treeItemState.id, (draft) => {
        draft.order = treeItemState.order;
        draft.expanded = treeItemState.expanded;
      });
    });
  },
  batchRemove: async (treeItemIds: string[], workspaceId: string) => {
    await sharedStorageIpc.batchRemoveItem(treeItemIds, {
      workspace: workspaceId ?? "application",
    });
  },
};

const constructTreeItemStateKey = (treeItemId: string, workspaceId: string) => {
  return `${SHARED_STORAGE_TREE_ITEM_STATE_KEY}.${workspaceId}.${treeItemId}`;
};
