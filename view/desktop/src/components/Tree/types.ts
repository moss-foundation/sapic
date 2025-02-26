export interface TreeProps {
  nodes: NodeProps[];
  onNodeUpdate?: (item: NodeProps) => void;
  onNodeExpand?: (node: NodeProps) => void;
  onNodeCollapse?: (node: NodeProps) => void;
  onTreeUpdate?: (nodes: NodeProps[]) => void;
  horizontalPadding?: number;
  nodeOffset?: number;
}

export interface RecursiveTreeProps {
  nodes: NodeProps[];
  onNodeUpdate?: (item: NodeProps) => void;
  onChildNodesUpdate?: (items: NodeProps[]) => void;
  onNodeExpand?: (node: NodeProps) => void;
  onNodeCollapse?: (node: NodeProps) => void;
  onTreeUpdate?: (nodes: NodeProps[]) => void;
  depth?: number;
  horizontalPadding: number;
  nodeOffset: number;
}

export interface TreeNodeProps {
  node: NodeProps;
  onNodeUpdate: (node: NodeProps) => void;
  onNodeExpand?: (node: NodeProps) => void;
  onNodeCollapse?: (node: NodeProps) => void;
  depth: number;
  horizontalPadding: number;
  nodeOffset: number;
}

export interface NodeProps {
  id: string | number;
  name: string;
  order: number;
  type: string;
  isExpanded: boolean;
  isFolder: boolean;
  childNodes: NodeProps[];
}
