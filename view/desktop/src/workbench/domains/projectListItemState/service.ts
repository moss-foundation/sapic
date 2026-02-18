import {
  getItemExpanded,
  removeItemExpanded,
  updateItemExpanded,
} from "@/workbench/usecases/sharedStorage/itemExpanded";

import { ProjectListItemState } from "./types";

const KEY = "projectList" as const;

interface IProjectListStateService {
  get: (workspaceId: string) => Promise<ProjectListItemState>;
  put: (projectListItemState: ProjectListItemState, workspaceId: string) => Promise<void>;
  remove: (workspaceId: string) => Promise<void>;
}

export const projectListStateService: IProjectListStateService = {
  get: async (workspaceId) => {
    const { value } = await getItemExpanded(KEY, workspaceId);
    if (value !== "none") {
      return { expanded: value.value as boolean };
    }
    return { expanded: false };
  },
  put: async (projectListItemState, workspaceId) => {
    await updateItemExpanded(KEY, projectListItemState.expanded, workspaceId);
  },
  remove: async (workspaceId) => {
    await removeItemExpanded(KEY, workspaceId);
  },
};
