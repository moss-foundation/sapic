import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { WorkspaceMode } from "@repo/base";
import { StreamProjectsEvent } from "@repo/ipc";
import { StreamResourcesEvent } from "@repo/moss-project";

import { ProjectDragType } from "./constants";

export interface ProjectTreeRootNode extends StreamProjectsEvent {
  childNodes: ProjectTreeNode[];
}

export interface ProjectTreeNode extends StreamResourcesEvent {
  childNodes: ProjectTreeNode[];
}

export interface ProjectTreeRootNodeProps {
  node: ProjectTreeRootNode;
}

export interface DragNode {
  projectId: string;
  repository?: string;
  node: ProjectTreeNode;
  parentNode: ProjectTreeNode;
}

export interface DropNode {
  projectId: string;
  repository?: string;
  node: ProjectTreeNode;
  parentNode: ProjectTreeNode | ProjectTreeRootNode;
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
