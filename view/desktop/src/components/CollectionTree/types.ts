import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { EntryInfo } from "@repo/moss-collection";
import { StreamCollectionsEvent, WorkspaceMode } from "@repo/moss-workspace";

export type SortTypes = "none" | "order" | "alphabetically";

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
  onNodeUpdate: (node: TreeCollectionNode) => void;
  node: TreeCollectionNode;
}

export interface DragNode {
  collectionId: string;
  repository?: string; //TODO This shouldn't be optional, I guess
  node: TreeCollectionNode;
  instruction?: Instruction;
}

export interface TreeProps {
  tree: TreeCollectionRootNode;

  paddingLeft?: number;
  paddingRight?: number;
  rootOffset?: number;
  nodeOffset?: number;
  searchInput?: string;
  sortBy?: SortTypes;
  displayMode?: WorkspaceMode;

  onTreeUpdate?: (tree: TreeCollectionRootNode) => void;
}

export interface TreeContextProps extends StreamCollectionsEvent {
  paddingLeft: number;
  paddingRight: number;
  rootOffset: number;
  nodeOffset: number;
  searchInput?: string;
  sortBy?: SortTypes;
  allFoldersAreCollapsed: boolean;
  allFoldersAreExpanded: boolean;
  displayMode: WorkspaceMode;
}

export interface TreeNodeComponentProps extends NodeEvents {
  node: TreeNodeProps;
  depth: number;
  parentNode: TreeNodeProps;
  isLastChild: boolean;
}

export interface NodeEvents {
  onNodeUpdate: (node: TreeNodeProps) => void;
}

export interface MoveNodeEventDetail {
  source: {
    node: TreeNodeProps;
    treeId: string;
  };
  target: {
    node: TreeNodeProps;
    treeId: string;
  };
  instruction?: Instruction;
}

export interface CreateNewCollectionFromTreeNodeEvent {
  source: {
    node: TreeNodeProps;
    treeId: string;
  };
}

export interface TreeNodeDropProps {
  type: "TreeNode";
  data: {
    node: TreeNodeProps;
    treeId: string;
  };
}

export interface DropNodeElement {
  node: TreeNodeProps;
  treeId: string;
}

export interface DropNodeElementWithInstruction {
  node: TreeNodeProps;
  treeId: string;
  instruction: Instruction;
}
