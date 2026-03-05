import { createPortal } from "react-dom";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import { ResourceNode } from "../../types";
import { ResourcesTreeNode } from "../ResourcesTreeNode";

interface ResourceNodePreviewProps {
  node: ResourceNode;
  preview: HTMLElement;
}

export const ResourceNodePreview = ({ node, preview }: ResourceNodePreviewProps) => {
  return createPortal(
    <ul className="background-(--moss-primary-background) flex gap-1 rounded-sm">
      <ResourcesTreeNode
        parentNode={{
          ...node,
          id: "-",
          name: "DraggedNode",
          order: undefined,
          expanded: false,
          childNodes: [],
        }}
        node={{ ...node, id: "DraggedNode", childNodes: [] }}
        depth={0}
      />
      <Icon icon="ChevronRight" className={cn("opacity-0")} />
    </ul>,
    preview
  );
};
