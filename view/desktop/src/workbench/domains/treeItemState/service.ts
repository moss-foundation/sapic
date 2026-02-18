import { projectSummariesCollection } from "@/db/projectSummaries/projectSummaries";
import {
  batchGetItemExpanded,
  batchPutItemExpanded,
  batchRemoveItemExpanded,
  getItemExpanded,
  removeItemExpanded,
  updateItemExpanded,
} from "@/workbench/usecases/sharedStorage/itemExpanded";
import {
  batchGetItemOrder,
  batchPutItemOrder,
  batchRemoveItemOrder,
  getItemOrder,
  putItemOrder,
  removeItemOrder,
} from "@/workbench/usecases/sharedStorage/itemOrder";

interface ITreeItemStateService {
  getOrder: (id: string, workspaceId: string) => Promise<number>;
  batchGetOrder: (ids: string[], workspaceId: string) => Promise<number[]>;
  putOrder: (id: string, order: number, workspaceId: string) => Promise<void>;
  batchPutOrder: (items: Record<string, number>, workspaceId: string) => Promise<void>;
  removeOrder: (id: string, workspaceId: string) => Promise<void>;
  batchRemoveOrder: (ids: string[], workspaceId: string) => Promise<void>;

  getExpanded: (id: string, workspaceId: string) => Promise<boolean>;
  batchGetExpanded: (ids: string[], workspaceId: string) => Promise<boolean[]>;
  putExpanded: (id: string, expanded: boolean, workspaceId: string) => Promise<void>;
  batchPutExpanded: (items: Record<string, boolean>, workspaceId: string) => Promise<void>;
  removeExpanded: (id: string, workspaceId: string) => Promise<void>;
  batchRemoveExpanded: (ids: string[], workspaceId: string) => Promise<void>;
}

export const treeItemStateService: ITreeItemStateService = {
  getOrder: async (id, workspaceId) => {
    const { value } = await getItemOrder(id, workspaceId);
    const order = value === "none" ? 0 : (value.value as number);
    if (projectSummariesCollection.has(id)) {
      projectSummariesCollection.update(id, (draft) => {
        draft.order = order;
      });
    }
    return order;
  },
  batchGetOrder: async (ids, workspaceId) => {
    const result = await batchGetItemOrder(ids, workspaceId);
    return ids.map((id) => (result.items[`${id}.order`] as number) ?? 0);
  },
  putOrder: async (id, order, workspaceId) => {
    await putItemOrder(id, order, workspaceId);
    if (projectSummariesCollection.has(id)) {
      projectSummariesCollection.update(id, (draft) => {
        draft.order = order;
      });
    }
  },
  batchPutOrder: async (items, workspaceId) => {
    await batchPutItemOrder(items, workspaceId);
    Object.entries(items).forEach(([id, order]) => {
      if (projectSummariesCollection.has(id)) {
        projectSummariesCollection.update(id, (draft) => {
          draft.order = order;
        });
      }
    });
  },
  removeOrder: async (id, workspaceId) => {
    await removeItemOrder(id, workspaceId);
    if (projectSummariesCollection.has(id)) {
      projectSummariesCollection.update(id, (draft) => {
        draft.order = undefined;
      });
    }
  },
  batchRemoveOrder: async (ids, workspaceId) => {
    await batchRemoveItemOrder(ids, workspaceId);
    ids.forEach((id) => {
      if (projectSummariesCollection.has(id)) {
        projectSummariesCollection.update(id, (draft) => {
          draft.order = undefined;
        });
      }
    });
  },

  getExpanded: async (id, workspaceId) => {
    const { value } = await getItemExpanded(id, workspaceId);
    const expanded = value === "none" ? false : (value.value as boolean);
    if (projectSummariesCollection.has(id)) {
      projectSummariesCollection.update(id, (draft) => {
        draft.expanded = expanded;
      });
    }
    return expanded;
  },
  batchGetExpanded: async (ids, workspaceId) => {
    const result = await batchGetItemExpanded(ids, workspaceId);
    return ids.map((id) => (result.items[`${id}.expanded`] as boolean) ?? false);
  },
  putExpanded: async (id, expanded, workspaceId) => {
    await updateItemExpanded(id, expanded, workspaceId);
    if (projectSummariesCollection.has(id)) {
      projectSummariesCollection.update(id, (draft) => {
        draft.expanded = expanded;
      });
    }
  },
  batchPutExpanded: async (items, workspaceId) => {
    await batchPutItemExpanded(items, workspaceId);
    Object.entries(items).forEach(([id, expanded]) => {
      if (projectSummariesCollection.has(id)) {
        projectSummariesCollection.update(id, (draft) => {
          draft.expanded = expanded;
        });
      }
    });
  },
  removeExpanded: async (id, workspaceId) => {
    await removeItemExpanded(id, workspaceId);
    if (projectSummariesCollection.has(id)) {
      projectSummariesCollection.update(id, (draft) => {
        draft.expanded = false;
      });
    }
  },
  batchRemoveExpanded: async (ids, workspaceId) => {
    await batchRemoveItemExpanded(ids, workspaceId);
    ids.forEach((id) => {
      if (projectSummariesCollection.has(id)) {
        projectSummariesCollection.update(id, (draft) => {
          draft.expanded = false;
        });
      }
    });
  },
};
