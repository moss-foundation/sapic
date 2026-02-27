import { ProjectTreeRootNode } from "../../types";

interface CalculateShouldRenderRootChildNodesProps {
  node: ProjectTreeRootNode;
  isAddingRootFileNode: boolean;
}

export const calculateShouldRenderRootChildNodes = ({
  node,
  isAddingRootFileNode,
}: CalculateShouldRenderRootChildNodesProps) => {
  if (isAddingRootFileNode) return true;
  return node.expanded;
};
