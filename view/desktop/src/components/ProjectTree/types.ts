import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { StreamEntriesEvent } from "@repo/moss-collection";
import { StreamCollectionsEvent, WorkspaceMode } from "@repo/moss-workspace";

import { ProjectDragType } from "./constants";

export interface ProjectTreeRootNode extends StreamCollectionsEvent {
  childNodes: ProjectTreeNode[];
}

export interface ProjectTreeNode extends StreamEntriesEvent {
  childNodes: ProjectTreeNode[];
}

export interface ProjectTreeRootNodeProps {
  node: ProjectTreeRootNode;
}

export interface DragNode {
  collectionId: string;
  repository?: string;
  node: ProjectTreeNode;
  parentNode: ProjectTreeNode;
}

export interface DropNode {
  collectionId: string;
  repository?: string;
  node: ProjectTreeNode;
  parentNode: ProjectTreeNode | ProjectTreeRootNode;
  instruction?: Instruction;
}

export interface DropRootNode {
  type: ProjectDragType.ROOT_NODE;
  collectionId: string;
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
}
