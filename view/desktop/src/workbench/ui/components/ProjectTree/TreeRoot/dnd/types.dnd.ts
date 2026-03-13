import { ProjectDragType } from "../../constants";
import { ResourcesTreeRoot } from "../../TreeRoot/types";
import { ProjectTreeRoot } from "../../types";

export interface DragTreeRootData {
  type: ProjectDragType.TREE_ROOT;
  data: {
    projectId: string;
    node: ProjectTreeRoot;
  };
  [key: string | symbol]: unknown;
}

export interface LocationResourcesListData {
  type: ProjectDragType.RESOURCES_LIST;
  data: {
    resourcesTree: ResourcesTreeRoot;
  };
  [key: string | symbol]: unknown;
}
