import { ProjectDragType } from "../../constants";
import { IResourcesTree, ProjectTree } from "../../types";

export interface DragTreeRootData {
  type: ProjectDragType.TREE_ROOT;
  data: {
    projectId: string;
    node: ProjectTree;
  };
  [key: string | symbol]: unknown;
}

export interface LocationResourcesListData {
  type: ProjectDragType.RESOURCES_LIST;
  data: {
    resourcesTree: IResourcesTree;
  };
  [key: string | symbol]: unknown;
}
