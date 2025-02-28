export interface TreeProps {
  tree: NodeProps;
  onNodeUpdate?: (node: NodeProps, oldId?: string | number) => void;
  onNodeExpand?: (node: NodeProps) => void;
  onNodeCollapse?: (node: NodeProps) => void;
  onTreeUpdate?: (nodes: NodeProps) => void;
  horizontalPadding?: number;
  nodeOffset?: number;
  sortBy?: "none" | "order" | "alphabetically";
  className?: string;

}

export interface RecursiveTreeProps {
  nodes: NodeProps[];
  onNodeUpdate?: (node: NodeProps, oldId?: string | number) => void;
  onChildNodesUpdate?: (nodes: NodeProps[]) => void;
  onNodeExpand?: (node: NodeProps) => void;
  onNodeCollapse?: (node: NodeProps) => void;
  onTreeUpdate?: (nodes: NodeProps) => void;
  depth?: number;
  horizontalPadding: number;
  nodeOffset: number;
}

export interface TreeNodeProps {
  node: NodeProps;
  onNodeUpdate: (node: NodeProps, oldId?: string | number) => void;
  onNodeExpand?: (node: NodeProps) => void;
  onNodeCollapse?: (node: NodeProps) => void;
  depth: number;
  horizontalPadding: number;
  nodeOffset: number;
}

export interface NodeProps {
  id: string | number;
  order: number;
  type: string;
  isExpanded: boolean;
  isFolder: boolean;
  childNodes: NodeProps[];
}
