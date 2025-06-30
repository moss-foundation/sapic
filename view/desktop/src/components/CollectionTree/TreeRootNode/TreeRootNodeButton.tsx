import { useContext } from "react";

import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

import { NodeLabel } from "../NodeLabel";
import { TreeContext } from "../Tree";
import { TreeCollectionRootNode } from "../types";

interface TreeRootNodeButtonProps {
  node: TreeCollectionRootNode;
  searchInput?: string;
  shouldRenderChildNodes: boolean;
  onRootNodeClick: (node: TreeCollectionRootNode) => void;
}

export const TreeRootNodeButton = ({
  node,
  searchInput,
  shouldRenderChildNodes,
  onRootNodeClick,
}: TreeRootNodeButtonProps) => {
  const { treeId, image } = useContext(TreeContext);
  const { api } = useTabbedPaneStore();

  const handleIconClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    onRootNodeClick({
      ...node,
      expanded: !node.expanded,
    });
  };

  const handleLabelClick = () => {
    const panel = api?.getPanel(treeId);

    if (!panel) {
      api?.addPanel({
        id: treeId,
        title: node.name,
        component: "CollectionSettings",
        params: {
          collectionId: treeId,
        },
      });

      onRootNodeClick({
        ...node,
        expanded: true,
      });
    } else {
      onRootNodeClick({
        ...node,
        expanded: !node.expanded,
      });
    }
  };

  return (
    <div
      className="group/treeRootNodeTrigger relative flex grow cursor-pointer items-center gap-1.5 overflow-hidden font-medium"
      onClick={handleLabelClick}
      role="button"
      tabIndex={0}
    >
      <span className="flex size-5 shrink-0 items-center justify-center">
        <button
          onClick={handleIconClick}
          className="hover:background-(--moss-icon-primary-background-hover) flex cursor-pointer items-center justify-center rounded-full"
        >
          <Icon
            icon="ChevronRight"
            className={cn("text-(--moss-icon-primary-text)", {
              "rotate-90": shouldRenderChildNodes,
              "hidden group-hover/treeRootNodeTrigger:block": image,
            })}
          />
        </button>

        {/* TODO: Replace with the actual image and don't forget to remove image from assets */}
        {image && (
          <div className="h-full w-full rounded group-hover/treeRootNodeTrigger:hidden">
            <img src={image} className="h-full w-full" />
          </div>
        )}
      </span>
      <NodeLabel label={node.name} searchInput={searchInput} />
    </div>
  );
};
