export type SortTypes = "none" | "order" | "alphabetically";

export interface Collection {
  id: number | string;
  "type": "collection";
  "order": number;
  "tree": NodeProps;
}

export interface NodeProps {
  id: string | number;
  type: string;
  order: number;
  isFolder: boolean;
  isExpanded: boolean;
  childNodes: NodeProps[];
}

export interface TreeNodeProps extends NodeProps {
  uniqueId: string;
  childNodes: TreeNodeProps[];
  isRoot: boolean;
}

export interface TreeProps {
  id?: string | number;
  tree: NodeProps;
  paddingLeft?: number;
  paddingRight?: number;
  nodeOffset?: number;
  searchInput?: string;
  onTreeUpdate?: (tree: NodeProps) => void;

  onRootAdd?: (node: TreeNodeProps) => void;
  onRootRemove?: (node: TreeNodeProps) => void;
  onRootRename?: (node: TreeNodeProps) => void;
  onRootUpdate?: (node: TreeNodeProps) => void;
  onRootClick?: (node: TreeNodeProps) => void;
  onRootDoubleClick?: (node: TreeNodeProps) => void;

  onNodeAdd?: (node: TreeNodeProps) => void;
  onNodeRemove?: (node: TreeNodeProps) => void;
  onNodeRename?: (node: TreeNodeProps) => void;
  onNodeUpdate?: (node: TreeNodeProps) => void;
  onNodeClick?: (node: TreeNodeProps) => void;
  onNodeDoubleClick?: (node: TreeNodeProps) => void;
}

export interface TreeContextProps {
  treeId: string | number;
  paddingLeft: number;
  paddingRight: number;
  nodeOffset: number;
  searchInput?: string;
  allFoldersAreCollapsed: boolean;
  allFoldersAreExpanded: boolean;
}

export interface TreeNodeComponentProps extends NodeEvents {
  node: TreeNodeProps;
  depth: number;
  parentNode: TreeNodeProps;

  onRootAdd?: (node: TreeNodeProps) => void;
  onRootRemove?: (node: TreeNodeProps) => void;
  onRootRename?: (node: TreeNodeProps) => void;
  onRootClick?: (node: TreeNodeProps) => void;
  onRootDoubleClick?: (node: TreeNodeProps) => void;

  onNodeAdd?: (node: TreeNodeProps) => void;
  onNodeRemove?: (node: TreeNodeProps) => void;
  onNodeRename?: (node: TreeNodeProps) => void;
  onNodeClick?: (node: TreeNodeProps) => void;
  onNodeDoubleClick?: (node: TreeNodeProps) => void;
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
