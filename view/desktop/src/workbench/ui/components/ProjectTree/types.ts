import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { WorkspaceMode } from "@repo/base";
import { ListProjectItem, ListProjectResourceItem } from "@repo/ipc";

export interface ProjectTree extends ListProjectItem {
  order?: number | undefined;
  expanded: boolean;
  resourcesTree: IResourcesTree;
  environmentsList: EnvironmentSummary[];
}

export interface ResourceNode extends ListProjectResourceItem {
  order?: number | undefined;
  expanded: boolean;
  childNodes: ResourceNode[];
}

export interface IResourcesTree {
  id: string;
  projectId: string;
  childNodes: ResourceNode[];
}

export interface DraggedResourceNode {
  projectId: string;
  node: ResourceNode;
  parentNode: ResourceNode | IResourcesTree;
}
export interface ProjectTreeProps {
  tree: ProjectTree;

  treePaddingLeft?: number;
  treePaddingRight?: number;
  nodeOffset?: number;
  searchInput?: string;
  displayMode?: WorkspaceMode;

  showOrders?: boolean;
  showTreeRootIds?: boolean;
}
