import {
  batchGetItemExpanded,
  batchPutItemExpanded,
  batchRemoveItemExpanded,
  getItemExpanded,
  removeItemExpanded,
  updateItemExpanded,
} from "@/workbench/usecases/sharedStorage/itemExpanded";

interface IEnvironmentListItemStateService {
  getExpanded: (id: string, workspaceId: string) => Promise<boolean | undefined>;
  batchGetExpanded: (ids: string[], workspaceId: string) => Promise<boolean[]>;
  putExpanded: (id: string, expanded: boolean, workspaceId: string) => Promise<void>;
  batchPutExpanded: (items: Record<string, boolean>, workspaceId: string) => Promise<void>;
  removeExpanded: (id: string, workspaceId: string) => Promise<void>;
  batchRemoveExpanded: (ids: string[], workspaceId: string) => Promise<void>;
}

export const environmentListItemStateService: IEnvironmentListItemStateService = {
  getExpanded: async (id, workspaceId) => {
    const { value } = await getItemExpanded(id, workspaceId);

    if (value === "none") return false;

    return value.value as unknown as boolean;
  },
  batchGetExpanded: async (ids, workspaceId) => {
    const { items } = await batchGetItemExpanded(ids, workspaceId);
    return ids.map((id) => (items[`${id}.expanded`] as boolean) ?? false);
  },
  putExpanded: async (id, expanded, workspaceId) => {
    await updateItemExpanded(id, expanded, workspaceId);
  },
  batchPutExpanded: async (items, workspaceId) => {
    await batchPutItemExpanded(items, workspaceId);
  },
  removeExpanded: async (id, workspaceId) => {
    await removeItemExpanded(id, workspaceId);
  },
  batchRemoveExpanded: async (ids, workspaceId) => {
    await batchRemoveItemExpanded(ids, workspaceId);
  },
} satisfies IEnvironmentListItemStateService;
