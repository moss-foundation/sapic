import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/list-item";
import { EntryInfo } from "@repo/moss-collection";

export type SortTypes = "none" | "order" | "alphabetically";

//TODO remove this after collections from backend are implemented
export interface CollectionTree {
  id: string;
  name: string;
  type: "collection";
  order: number | null;
  tree: TreeCollectionRootNode;
}

export interface TreeCollectionRootNode {
  id: string;
  name: string;
  order: number | null;
  expanded: boolean;
  endpoints: TreeCollectionNode;
  schemas: TreeCollectionNode;
  components: TreeCollectionNode;
  requests: TreeCollectionNode;
}

export interface TreeCollectionNode extends EntryInfo {
  childNodes: TreeCollectionNode[];
}

export interface TreeNodeComponentPropsV2 {
  node: TreeCollectionRootNode;
  depth: number;
  parentNode: TreeCollectionRootNode;
  isLastChild: boolean;
  onNodeUpdate: (node: TreeCollectionRootNode) => void;
}

export interface TreeRootNodeProps {
  onNodeUpdate: (node: TreeCollectionNode) => void;
  node: TreeCollectionNode;
}

export interface TreeProps {
  tree: TreeCollectionRootNode;
  image: string | undefined;

  paddingLeft?: number;
  paddingRight?: number;
  rootOffset?: number;
  nodeOffset?: number;
  searchInput?: string;
  sortBy?: SortTypes;
  displayMode?: "RequestFirst" | "DesignFirst";

  onTreeUpdate?: (tree: TreeCollectionRootNode) => void;

  onRootAdd?: (node: TreeCollectionRootNode) => void;
  onRootRemove?: (node: TreeCollectionRootNode) => void;
  onRootRename?: (node: TreeCollectionRootNode) => void;
  onRootUpdate?: (node: TreeCollectionRootNode) => void;
  onRootClick?: (node: TreeCollectionRootNode) => void;
  onRootDoubleClick?: (node: TreeCollectionRootNode) => void;

  onNodeAdd?: (node: TreeCollectionNode) => void;
  onNodeRemove?: (node: TreeCollectionNode) => void;
  onNodeRename?: (node: TreeCollectionNode) => void;
  onNodeUpdate?: (node: TreeCollectionNode) => void;
  onNodeClick?: (node: TreeCollectionNode) => void;
  onNodeDoubleClick?: (node: TreeCollectionNode) => void;
}

export interface TreeContextProps {
  treeId: string;
  image: string | undefined;
  paddingLeft: number;
  paddingRight: number;
  rootOffset: number;
  nodeOffset: number;
  searchInput?: string;
  sortBy?: SortTypes;
  allFoldersAreCollapsed: boolean;
  allFoldersAreExpanded: boolean;
  displayMode: "RequestFirst" | "DesignFirst";
  onRootAddCallback?: (node: TreeCollectionRootNode) => void;
  onRootRemoveCallback?: (node: TreeCollectionRootNode) => void;
  onRootRenameCallback?: (node: TreeCollectionRootNode) => void;
  onRootUpdateCallback?: (node: TreeCollectionRootNode) => void;
  onRootClickCallback?: (node: TreeCollectionRootNode) => void;
  onRootDoubleClickCallback?: (node: TreeCollectionRootNode) => void;
  onNodeAddCallback?: (node: TreeCollectionNode) => void;
  onNodeRemoveCallback?: (node: TreeCollectionNode) => void;
  onNodeRenameCallback?: (node: TreeCollectionNode) => void;
  onNodeUpdateCallback?: (node: TreeCollectionNode) => void;
  onNodeClickCallback?: (node: TreeCollectionNode) => void;
  onNodeDoubleClickCallback?: (node: TreeCollectionNode) => void;
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
