import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";

import { WorkspaceListState } from "./types";

const SHARED_STORAGE_WORKSPACE_LIST_STATE_KEY = "workbench.workspaceListState" as const;

interface IWorkspaceListStateService {
  get: (workspaceId: string) => Promise<WorkspaceListState>;
  put: (workspaceListState: WorkspaceListState, workspaceId: string) => Promise<void>;
  remove: (workspaceId: string) => Promise<void>;
}

export const workspaceListStateService: IWorkspaceListStateService = {
  get: async (workspaceId: string) => {
    const { value: output } = await sharedStorageIpc.getItem(constructWorkspaceListStateKey(workspaceId), {
      workspace: workspaceId,
    });

    if (output !== "none") {
      return output.value as unknown as WorkspaceListState;
    }

    return { expanded: false } satisfies WorkspaceListState;
  },
  put: async (workspaceListState: WorkspaceListState, workspaceId: string) => {
    const { ...state } = workspaceListState;
    await sharedStorageIpc.putItem(constructWorkspaceListStateKey(workspaceId), state, {
      workspace: workspaceId,
    });
  },
  remove: async (workspaceId: string) => {
    await sharedStorageIpc.removeItem(constructWorkspaceListStateKey(workspaceId), {
      workspace: workspaceId,
    });
  },
};

const constructWorkspaceListStateKey = (workspaceId: string) => {
  return `${SHARED_STORAGE_WORKSPACE_LIST_STATE_KEY}.${workspaceId}`;
};
