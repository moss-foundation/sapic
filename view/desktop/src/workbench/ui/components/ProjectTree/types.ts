import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { WorkspaceMode } from "@repo/base";
import { ListProjectItem, ListProjectResourceItem } from "@repo/ipc";

import { ProjectDragType } from "./constants";

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

export interface DragNode {
  projectId: string;
  node: ResourceNode;
  parentNode: ResourceNode | IResourcesTree;
}

export interface DropNode {
  projectId: string;
  node: ResourceNode;
  parentNode: ResourceNode | IResourcesTree;
}

export interface DropRootNode {
  type: ProjectDragType.ROOT_NODE;
  projectId: string;
  repository?: string;
  node: ProjectTree;
  instruction?: Instruction;
}

export interface ProjectTreeProps {
  tree: ProjectTree;

  treePaddingLeft?: number;
  treePaddingRight?: number;
  nodeOffset?: number;
  searchInput?: string;
  displayMode?: WorkspaceMode;

  showOrders?: boolean;
  showRootNodeIds?: boolean;
}
