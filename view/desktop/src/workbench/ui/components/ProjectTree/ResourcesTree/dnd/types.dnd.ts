import { DescribeResourceOutput } from "@repo/moss-project";

import { ProjectDragType } from "../../constants";
import { ResourcesTreeRoot } from "../../TreeRoot/types";
import { ResourceNode } from "../types";

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
  data: DragResourceNodeData;
  [key: string | symbol]: unknown;
}
export interface DragResourceNodeData {
  projectId: string;
  node: ResourceNode;
  parentNode: ResourceNode | ResourcesTreeRoot;
}

export interface ResourceNodeWithDetails extends ResourceNode {
  details: DescribeResourceOutput;
}
