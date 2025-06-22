import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

import TestMossImage from "../../../assets/images/TestMossImage.webp";
import { NodeLabel } from "../NodeLabel";
import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";

interface TreeRootNodeButtonProps {
  node: TreeCollectionRootNode;
  searchInput?: string;
  shouldRenderChildNodes: boolean;
  handleRootNodeClick: (node: TreeCollectionRootNode) => void;
}

export const TreeRootNodeButton = ({
  node,
  searchInput,
  shouldRenderChildNodes,
  handleRootNodeClick,
}: TreeRootNodeButtonProps) => {
  const { onRootClickCallback, onRootDoubleClickCallback } = useContext(TreeContext);

  return (
    <button
      className="group/treeRootNodeTrigger relative flex grow cursor-pointer items-center gap-1.5 overflow-hidden font-medium"
      onClick={() => {
        handleRootNodeClick({
          ...node,
          expanded: !node.expanded,
        });
        onRootClickCallback?.(node);
      }}
      onDoubleClick={() => onRootDoubleClickCallback?.(node)}
    >
      <span className="flex size-5 shrink-0 items-center justify-center">
        <Icon
          icon="ChevronRight"
          className={cn("text-(--moss-icon-primary-text)", {
            "rotate-90": shouldRenderChildNodes,
            "hidden group-hover/treeRootNodeTrigger:block": TestMossImage,
          })}
        />

        {/* TODO: Replace with the actual image and don't forget to remove image from assets */}
        {TestMossImage && (
          <div className="h-full w-full rounded group-hover/treeRootNodeTrigger:hidden">
            <img src={TestMossImage} className="h-full w-full" />
          </div>
        )}
      </span>
      <NodeLabel label={node.name} searchInput={searchInput} />{" "}
    </button>
  );
};
