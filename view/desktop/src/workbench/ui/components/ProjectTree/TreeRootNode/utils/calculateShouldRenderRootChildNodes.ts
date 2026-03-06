import { ProjectTree } from "../../types";

interface CalculateShouldRenderRootChildNodesProps {
  node: ProjectTree;
  isAddingRootFileNode: boolean;
}

export const calculateShouldRenderRootChildNodes = ({
  node,
  isAddingRootFileNode,
}: CalculateShouldRenderRootChildNodesProps) => {
  if (isAddingRootFileNode) return true;
  return node.expanded;
};
