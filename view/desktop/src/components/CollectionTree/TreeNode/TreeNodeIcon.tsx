import { Icon } from "@/lib/ui/Icon";

import { TreeCollectionNode } from "../types";

interface TreeNodeIconProps {
  node: TreeCollectionNode;
  isRootNode: boolean;
  className?: string;
}

export const TreeNodeIcon = ({ node, isRootNode, className }: TreeNodeIconProps) => {
  if (isRootNode) {
    switch (node.class) {
      case "Schema":
        return <Icon icon="SchemasFolder" className={className} />;
      case "Endpoint":
        return <Icon icon="EndpointsFolder" className={className} />;
      case "Component":
        return <Icon icon="ComponentsFolder" className={className} />;
      default:
        return <Icon icon="RequestsFolder" className={className} />;
    }
  }

  switch (node.class) {
    case "Schema":
      return <Icon icon="Schema" className={className} />;
    case "Endpoint":
      return <Icon icon="Endpoint" className={className} />;
    case "Component":
      return <Icon icon="Component" className={className} />;
    default:
      return <Icon icon="Request" className={className} />;
  }
};
