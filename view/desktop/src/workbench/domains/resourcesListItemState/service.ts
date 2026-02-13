import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";

import { ResourcesListItemState } from "./types";

const SHARED_STORAGE_RESOURCES_LIST_ITEM_STATE_KEY = "workbench.resourcesListItemState" as const;

interface IResourcesListItemStateService {
  get: (resourcesListItemId: string, workspaceId: string) => Promise<ResourcesListItemState>;
  put: (
    resourcesListItemId: string,
    resourcesListItemState: ResourcesListItemState,
    workspaceId: string
  ) => Promise<void>;
  remove: (resourcesListItemId: string, workspaceId: string) => Promise<void>;
}

export const resourcesListItemStateService: IResourcesListItemStateService = {
  get: async (resourcesListItemId: string, workspaceId: string) => {
    const { value: output } = await sharedStorageIpc.getItem(
      constructResourcesListItemStateKey(resourcesListItemId, workspaceId),
      { workspace: workspaceId }
    );

    if (output !== "none") {
      return output.value as unknown as ResourcesListItemState;
    }

    return { expanded: false } satisfies ResourcesListItemState;
  },
  put: async (resourcesListItemId: string, resourcesListItemState: ResourcesListItemState, workspaceId: string) => {
    const { ...state } = resourcesListItemState;
    await sharedStorageIpc.putItem(constructResourcesListItemStateKey(resourcesListItemId, workspaceId), state, {
      workspace: workspaceId,
    });
  },
  remove: async (resourcesListItemId: string, workspaceId: string) => {
    await sharedStorageIpc.removeItem(constructResourcesListItemStateKey(resourcesListItemId, workspaceId), {
      workspace: workspaceId,
    });
  },
};

const constructResourcesListItemStateKey = (resourcesListItemId: string, workspaceId: string) => {
  return `${SHARED_STORAGE_RESOURCES_LIST_ITEM_STATE_KEY}.${workspaceId}.${resourcesListItemId}`;
};
