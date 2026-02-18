import {
  getItemExpanded,
  removeItemExpanded,
  updateItemExpanded,
} from "@/workbench/usecases/sharedStorage/itemExpanded";

interface IResourcesListItemStateService {
  get: (resourcesListItemId: string, workspaceId: string) => Promise<boolean>;
  put: (resourcesListItemId: string, resourcesListItemState: boolean, workspaceId: string) => Promise<void>;
  remove: (resourcesListItemId: string, workspaceId: string) => Promise<void>;
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
  put: async (resourcesListItemId, resourcesListItemState, workspaceId) => {
    const key = `resourcesListItem.${resourcesListItemId}`;
    await updateItemExpanded(key, resourcesListItemState, workspaceId);
  },
  remove: async (resourcesListItemId, workspaceId) => {
    const key = `resourcesListItem.${resourcesListItemId}`;
    await removeItemExpanded(key, workspaceId);
  },
};
