import { ProjectDragType } from "../../constants";
import { ResourceNode } from "../../types";

export interface LocationResourcesListData {
  type: ProjectDragType.RESOURCES_LIST;
  data: {
    projectId: string;
    rootResourcesNodes: ResourceNode[];
  };
  [key: string | symbol]: unknown;
}

export interface DragResourceNode {
  type: ProjectDragType.NODE;
  data: {
    projectId: string;
    node: ResourceNode;
    parentNode?: ResourceNode;
  };
  [key: string | symbol]: unknown;
}
