import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { EntryInfo } from "@repo/moss-collection";
import { StreamCollectionsEvent, WorkspaceMode } from "@repo/moss-workspace";

export interface TreeCollectionRootNode extends StreamCollectionsEvent {
  expanded: boolean;
  endpoints: TreeCollectionNode;
  schemas: TreeCollectionNode;
  components: TreeCollectionNode;
  requests: TreeCollectionNode;
}

export interface TreeCollectionNode extends EntryInfo {
  childNodes: TreeCollectionNode[];
}

export interface TreeRootNodeProps {
  node: TreeCollectionRootNode;
}

export interface DragNode {
  collectionId: string;
  repository: string;
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
}

export interface DropNode {
  collectionId: string;
  repository: string;
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
  instruction?: Instruction;
}

export interface DropRootNode {
  type: "TreeRootNode";
  collectionId: string;
  node: TreeCollectionRootNode;
  instruction?: Instruction;
}

export interface TreeProps {
  tree: TreeCollectionRootNode;

  treePaddingLeft?: number;
  treePaddingRight?: number;
  nodeOffset?: number;
  searchInput?: string;
  displayMode?: WorkspaceMode;
  showNodeOrders?: boolean;

  onTreeUpdate?: (tree: TreeCollectionRootNode) => void;
}

export interface TreeContextProps extends StreamCollectionsEvent {
  treePaddingLeft: number;
  treePaddingRight: number;
  nodeOffset: number;
  searchInput?: string;
  allFoldersAreCollapsed: boolean;
  allFoldersAreExpanded: boolean;
  displayMode: WorkspaceMode;
  showNodeOrders: boolean;
}
