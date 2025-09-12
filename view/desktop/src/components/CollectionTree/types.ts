import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { StreamEntriesEvent } from "@repo/moss-collection";
import { StreamCollectionsEvent, WorkspaceMode } from "@repo/moss-workspace";

export interface TreeCollectionRootNode extends StreamCollectionsEvent {
  expanded: boolean;
  childNodes: TreeCollectionNode[];
}

export interface TreeCollectionNode extends StreamEntriesEvent {
  childNodes: TreeCollectionNode[];
}

export interface TreeRootNodeProps {
  node: TreeCollectionRootNode;
}

export interface DragNode {
  collectionId: string;
  repository?: string;
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
}

export interface DropNode {
  collectionId: string;
  repository?: string;
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
  instruction?: Instruction;
}

export interface DropRootNode {
  type: "TreeRootNode";
  collectionId: string;
  repository?: string;
  node: TreeCollectionRootNode;
  instruction?: Instruction;
}

export interface CollectionTreeProps {
  tree: TreeCollectionRootNode;

  treePaddingLeft?: number;
  treePaddingRight?: number;
  nodeOffset?: number;
  searchInput?: string;
  displayMode?: WorkspaceMode;
  showOrders?: boolean;
}
