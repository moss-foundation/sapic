import {
  getItemExpanded,
  removeItemExpanded,
  updateItemExpanded,
} from "@/workbench/usecases/sharedStorage/itemExpanded";

const KEY = "projectList" as const;

interface IProjectListStateService {
  get: (workspaceId: string) => Promise<boolean>;
  put: (expanded: boolean, workspaceId: string) => Promise<void>;
  remove: (workspaceId: string) => Promise<void>;
}

export const projectListStateService: IProjectListStateService = {
  get: async (workspaceId) => {
    const { value } = await getItemExpanded(KEY, workspaceId);
    if (value !== "none") {
      return value.value as boolean;
    }
    return false;
  },
  put: async (expanded, workspaceId) => {
    await updateItemExpanded(KEY, expanded, workspaceId);
  },
  remove: async (workspaceId) => {
    await removeItemExpanded(KEY, workspaceId);
  },
};
