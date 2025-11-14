import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";
import { QueryParamInfo } from "@repo/moss-project";

import { DRAGGABLE_PARAM_ROW_TYPE, DROP_TARGET_NEW_PARAM_ROW_FORM_TYPE, DROP_TARGET_PARAM_ROW_TYPE } from "./constants";

export type ParamDragType = "query" | "path";

export interface DraggableParamRowData {
  type: typeof DRAGGABLE_PARAM_ROW_TYPE;
  data: {
    param: QueryParamInfo;
    paramType: ParamDragType;
    resourceId: string;
  };
  [key: string]: unknown;
}

export interface DropTargetParamRowData {
  type: typeof DROP_TARGET_PARAM_ROW_TYPE;
  edge: Edge | null;
  data: {
    param: QueryParamInfo;
    paramType: ParamDragType;
    resourceId: string;
  };
  [key: string]: unknown;
}

export interface DropTargetNewParamRowFormData {
  type: typeof DROP_TARGET_NEW_PARAM_ROW_FORM_TYPE;
  data: {
    resourceId: string;
    paramType: ParamDragType;
  };
  [key: string]: unknown;
}
