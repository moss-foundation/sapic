import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Availability } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../../constants";
import { DragEnvironmentItem } from "../../types.dnd";

export const canReorder = (
  sourceData: DragEnvironmentItem,
  locationData: DragEnvironmentItem,
  workspaceEnvironments: EnvironmentSummary[],
  allProjectEnvironments: EnvironmentSummary[]
): Availability => {
  const isSourceWorkspace = sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE;
  const isTargetWorkspace = locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE;
  const isSourceProject = sourceData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT;
  const isTargetProject = locationData.type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT;

  //Workspace --> Workspace
  if (isSourceWorkspace && isTargetWorkspace) {
    return "available";
  }

  //Workspace -> Project
  if (isSourceWorkspace && isTargetProject) {
    const projectEnvs = allProjectEnvironments.filter((env) => env.projectId === locationData.data.projectId);
    const hasConflict = hasSameNameOrId(projectEnvs, sourceData.data.name, sourceData.data.id);
    return hasConflict ? "not-available" : "available";
  }

  //Project -> Project
  if (isSourceProject && isTargetProject) {
    if (sourceData.data.projectId === locationData.data.projectId) {
      return "available";
    }
    if (sourceData.data.projectId !== locationData.data.projectId) {
      const projectEnvs = allProjectEnvironments.filter((env) => env.projectId === locationData.data.projectId);
      const hasConflict = hasSameNameOrId(projectEnvs, sourceData.data.name, sourceData.data.id);
      return hasConflict ? "not-available" : "available";
    }
  }

  //Project -> Workspace
  if (isSourceProject && isTargetWorkspace) {
    const hasConflict = hasSameNameOrId(workspaceEnvironments, sourceData.data.name, sourceData.data.id);
    return hasConflict ? "not-available" : "available";
  }
  //Workspace -> Project List
  return "not-available";
};

const hasSameNameOrId = (environments: EnvironmentSummary[], name: string, id: string) => {
  return environments.some((env) => env.name === name || env.id === id);
};
