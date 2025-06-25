import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

import TestMossImage from "../../../assets/images/TestMossImage.webp";
import { NodeLabel } from "../NodeLabel";
import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";

interface TreeRootNodeButtonProps {
  node: TreeCollectionRootNode;
  searchInput?: string;
  shouldRenderChildNodes: boolean;
  onFolderToggleClick: (node: TreeCollectionRootNode) => void;
}

export const TreeRootNodeButton = ({
  node,
  searchInput,
  shouldRenderChildNodes,
  onFolderToggleClick,
}: TreeRootNodeButtonProps) => {
  const { treeId } = useContext(TreeContext);
  const { addOrFocusPanel } = useTabbedPaneStore();

  const handleIconClick = (e: React.MouseEvent<SVGSVGElement>) => {
    e.stopPropagation();
    onFolderToggleClick({
      ...node,
      expanded: !node.expanded,
    });
  };

  const handleLabelClick = () => {
    addOrFocusPanel({
      id: treeId,
      title: node.name,
      component: "CollectionSettings",
      params: {
        collectionId: treeId,
      },
    });
  };

  return (
    <button
      className="group/treeRootNodeTrigger relative flex grow cursor-pointer items-center gap-1.5 overflow-hidden font-medium"
      onClick={handleLabelClick}
    >
      <span className="flex size-5 shrink-0 items-center justify-center">
        <Icon
          icon="ChevronRight"
          className={cn("text-(--moss-icon-primary-text)", {
            "rotate-90": shouldRenderChildNodes,
            "hidden group-hover/treeRootNodeTrigger:block": TestMossImage,
          })}
          onClick={handleIconClick}
        />

        {/* TODO: Replace with the actual image and don't forget to remove image from assets */}
        {TestMossImage && (
          <div className="h-full w-full rounded group-hover/treeRootNodeTrigger:hidden">
            <img src={TestMossImage} className="h-full w-full" />
          </div>
        )}
      </span>
      <NodeLabel label={node.name} searchInput={searchInput} />
    </button>
  );
};
