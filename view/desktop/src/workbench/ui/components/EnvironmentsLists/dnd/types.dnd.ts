import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ENVIRONMENT_ITEM_DRAG_TYPE, ENVIRONMENT_LIST_DRAG_TYPE } from "../constants";

export interface DragEnvironmentItem {
  type: ENVIRONMENT_ITEM_DRAG_TYPE;
  data: EnvironmentSummary;
  [key: string | symbol]: unknown;
}

export interface DropEnvironmentItem extends DragEnvironmentItem {
  instruction?: Instruction;
}

export interface DropProjectEnvironmentList {
  type: ENVIRONMENT_LIST_DRAG_TYPE.PROJECT;
  data: {
    projectId: string;
    projectEnvironments: EnvironmentSummary[];
  };
  [key: string | symbol]: unknown;
}

export interface DropWorkspaceEnvironmentList {
  type: ENVIRONMENT_LIST_DRAG_TYPE.WORKSPACE;
  data: {
    workspaceEnvironments: EnvironmentSummary[];
  };
  [key: string | symbol]: unknown;
}
