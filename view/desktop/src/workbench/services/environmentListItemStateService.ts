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
    const key = `environmentListItem.${id}`;
    const { value } = await getItemExpanded(key, workspaceId);

    if (value === "none") return false;

    return value.value as unknown as boolean;
  },
  batchGetExpanded: async (ids, workspaceId) => {
    const keys = ids.map((id) => `environmentListItem.${id}`);
    const { items } = await batchGetItemExpanded(keys, workspaceId);
    return ids.map((id) => (items[`environmentListItem.${id}`] as boolean) ?? false);
  },
  putExpanded: async (id, expanded, workspaceId) => {
    const key = `environmentListItem.${id}`;
    await updateItemExpanded(key, expanded, workspaceId);
  },
  batchPutExpanded: async (items, workspaceId) => {
    const keys = Object.keys(items).map((id) => `environmentListItem.${id}`);
    const values = Object.values(items);
    await batchPutItemExpanded(Object.fromEntries(keys.map((key, index) => [`${key}`, values[index]])), workspaceId);
  },
  removeExpanded: async (id, workspaceId) => {
    const key = `environmentListItem.${id}`;
    await removeItemExpanded(key, workspaceId);
  },
  batchRemoveExpanded: async (ids, workspaceId) => {
    const keys = ids.map((id) => `environmentListItem.${id}`);
    await batchRemoveItemExpanded(keys, workspaceId);
  },
} satisfies IEnvironmentListItemStateService;
