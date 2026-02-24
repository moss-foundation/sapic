import { ProjectDragType } from "../constants";
import { ProjectTreeRootNode } from "../types";

export interface DragTreeRootNodeData {
  type: ProjectDragType.ROOT_NODE;
  data: {
    projectId: string;
    node: ProjectTreeRootNode;
  };
  [key: string | symbol]: unknown;
}
