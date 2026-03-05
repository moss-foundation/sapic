import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { WorkspaceMode } from "@repo/base";
import { ListProjectItem, ListProjectResourceItem } from "@repo/ipc";

import { ProjectDragType } from "./constants";

export interface ProjectTreeRootNode extends ListProjectItem {
  order?: number | undefined;
  expanded: boolean;
  childNodes: ResourceNode[];
}

export interface ResourceNode extends ListProjectResourceItem {
  order?: number | undefined;
  expanded: boolean;
  childNodes: ResourceNode[];
}

export interface DragNode {
  projectId: string;
  node: ResourceNode;
  parentNode: ResourceNode;
}

export interface DropNode {
  projectId: string;
  node: ResourceNode;
  parentNode: ResourceNode | ProjectTreeRootNode;
  instruction?: Instruction;
}

export interface DropRootNode {
  type: ProjectDragType.ROOT_NODE;
  projectId: string;
  repository?: string;
  node: ProjectTreeRootNode;
  instruction?: Instruction;
}

export interface ProjectTreeProps {
  tree: ProjectTreeRootNode;

  treePaddingLeft?: number;
  treePaddingRight?: number;
  nodeOffset?: number;
  searchInput?: string;
  displayMode?: WorkspaceMode;

  showOrders?: boolean;
  showRootNodeIds?: boolean;
}
