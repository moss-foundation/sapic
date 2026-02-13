import { sharedStorageIpc } from "@/infra/ipc/sharedStorageIpc";

import { ProjectListItemState } from "./types";

const SHARED_STORAGE_PROJECT_LIST_STATE_KEY = "workbench.projectListState" as const;

interface IProjectListStateService {
  get: (workspaceId: string) => Promise<ProjectListItemState>;
  put: (projectListItemState: ProjectListItemState, workspaceId: string) => Promise<void>;
  remove: (workspaceId: string) => Promise<void>;
}

export const projectListStateService: IProjectListStateService = {
  get: async (workspaceId: string) => {
    const { value: output } = await sharedStorageIpc.getItem(constructProjectListStateKey(workspaceId), {
      workspace: workspaceId,
    });

    if (output !== "none") {
      return output.value as unknown as ProjectListItemState;
    }

    return { expanded: false } satisfies ProjectListItemState;
  },
  put: async (projectListItemState: ProjectListItemState, workspaceId: string) => {
    const { ...state } = projectListItemState;
    await sharedStorageIpc.putItem(constructProjectListStateKey(workspaceId), state, {
      workspace: workspaceId,
    });
  },
  remove: async (workspaceId: string) => {
    await sharedStorageIpc.removeItem(constructProjectListStateKey(workspaceId), {
      workspace: workspaceId,
    });
  },
};

const constructProjectListStateKey = (workspaceId: string) => {
  return `${SHARED_STORAGE_PROJECT_LIST_STATE_KEY}.${workspaceId}`;
};
