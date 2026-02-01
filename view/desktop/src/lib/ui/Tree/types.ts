export interface BaseTreeContextProps {
  id: string;
  name: string;
  order: number;
  treePaddingLeft: number;
  treePaddingRight: number;
  nodeOffset: number;
  showOrders?: boolean;
  showRootNodeIds?: boolean;
}
