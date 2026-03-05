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

export interface LocationResourcesListData {
  type: ProjectDragType.RESOURCES_LIST;
  data: {
    tree: ProjectTreeRootNode;
  };
  [key: string | symbol]: unknown;
}
