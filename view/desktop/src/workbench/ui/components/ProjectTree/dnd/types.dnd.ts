import { ProjectDragType } from "../constants";
import { ProjectTreeRootNode, ResourcesTree } from "../types";

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
    resourcesTree: ResourcesTree;
  };
  [key: string | symbol]: unknown;
}
