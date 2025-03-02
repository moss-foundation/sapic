export interface TreeNodeDropProps {
  type: "TreeNode";
  data: {
    node: TreeNodeProps;
    treeId: string;
  };
}

export interface TreeProps {
  tree: NodeProps;
  horizontalPadding?: number;
  nodeOffset?: number;
  onTreeUpdate?: (tree: NodeProps) => void;
}

export interface NodeEvents {
  onNodeUpdate: (node: TreeNodeProps) => void;
}

export interface TreeNodeComponentProps extends NodeEvents {
  node: TreeNodeProps;
  depth: number;
  horizontalPadding: number;
  nodeOffset: number;
  treeId: string;
}

export interface TreeNodeProps extends NodeProps {
  uniqueId: string;
  childNodes: TreeNodeProps[];

}

export interface NodeProps {
  id: string | number;
  type: string;
  order: number;
  isFolder: boolean;
  isExpanded: boolean;
  childNodes: NodeProps[];
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
export interface DropNodeElement {
  node: TreeNodeProps; treeId: string
}
export type SortTypes = "none" | "order" | "alphabetically";