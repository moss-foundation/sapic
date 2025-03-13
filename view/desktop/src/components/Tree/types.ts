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
  horizontalPadding?: number;
  nodeOffset?: number;
  searchInput?: string;
  onTreeUpdate?: (tree: NodeProps) => void;
}

export interface TreeContextProps {
  treeId: string | number;
  horizontalPadding: number;
  nodeOffset: number;
  searchInput?: string;
  allFoldersAreCollapsed: boolean;
  allFoldersAreExpanded: boolean;
}

export interface TreeNodeComponentProps extends NodeEvents {
  node: TreeNodeProps;
  depth: number;
  parentNode: TreeNodeProps;
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
