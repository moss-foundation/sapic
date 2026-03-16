import {
  batchGetItemExpanded,
  batchPutItemExpanded,
  batchRemoveItemExpanded,
  getItemExpanded,
  removeItemExpanded,
  updateItemExpanded,
} from "@/workbench/usecases/sharedStorage/itemExpanded";

interface IResourcesListItemStateService {
  get: (resourcesListItemId: string, workspaceId: string) => Promise<boolean>;
  batchGet: (ids: string[], workspaceId: string) => Promise<boolean[]>;
  put: (resourcesListItemId: string, resourcesListItemState: boolean, workspaceId: string) => Promise<void>;
  batchPut: (items: Record<string, boolean>, workspaceId: string) => Promise<void>;
  remove: (resourcesListItemId: string, workspaceId: string) => Promise<void>;
  batchRemove: (ids: string[], workspaceId: string) => Promise<void>;
}

export const resourcesListItemStateService: IResourcesListItemStateService = {
  get: async (resourcesListItemId, workspaceId) => {
    const key = `resourcesListItem.${resourcesListItemId}`;
    const { value } = await getItemExpanded(key, workspaceId);
    if (value !== "none") {
      return value.value as boolean;
    }
    return false;
  },
  batchGet: async (ids, workspaceId) => {
    const keys = ids.map((id) => `resourcesListItem.${id}`);
    const { items } = await batchGetItemExpanded(keys, workspaceId);
    return ids.map((id) => (items[`resourcesListItem.${id}`] as boolean) ?? false);
  },
  put: async (resourcesListItemId, resourcesListItemState, workspaceId) => {
    const key = `resourcesListItem.${resourcesListItemId}`;
    await updateItemExpanded(key, resourcesListItemState, workspaceId);
  },
  batchPut: async (items, workspaceId) => {
    const keys = Object.keys(items).map((id) => `resourcesListItem.${id}`);
    const values = Object.values(items);
    await batchPutItemExpanded(Object.fromEntries(keys.map((key, index) => [`${key}`, values[index]])), workspaceId);
  },
  remove: async (resourcesListItemId, workspaceId) => {
    const key = `resourcesListItem.${resourcesListItemId}`;
    await removeItemExpanded(key, workspaceId);
  },
  batchRemove: async (ids, workspaceId) => {
    const keys = ids.map((id) => `resourcesListItem.${id}`);
    await batchRemoveItemExpanded(keys, workspaceId);
  },
} satisfies IResourcesListItemStateService;
