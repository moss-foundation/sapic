import { Icon } from "@/lib/ui/Icon";

import { TreeCollectionNode } from "../types";

interface TreeNodeIconProps {
  node: TreeCollectionNode;
  className?: string;
}

export const TreeNodeIcon = ({ node, className }: TreeNodeIconProps) => {
  const calculateIsRoot = node.path.segments.length === 1;

  if (calculateIsRoot) {
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

  if (node.kind === "Dir") {
    return <Icon icon="Folder" className={className} />;
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
