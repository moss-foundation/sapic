import { useContext } from "react";

import { useUpdateCollection } from "@/hooks";
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
}

export const TreeRootNodeButton = ({ node, searchInput, shouldRenderChildNodes }: TreeRootNodeButtonProps) => {
  const { id, picturePath, showNodeOrders } = useContext(TreeContext);
  const { api } = useTabbedPaneStore();
  const { mutateAsync: updateCollection } = useUpdateCollection();

  const handleIconClick = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    await updateCollection({
      id,
      expanded: !node.expanded,
    });
  };

  const handleLabelClick = async () => {
    const panel = api?.getPanel(id);

    if (!panel) {
      api?.addPanel({
        id,
        title: node.name,
        component: "CollectionSettings",
        params: {
          collectionId: id,
        },
      });

      await updateCollection({
        id,
        expanded: true,
      });
    } else {
      await updateCollection({
        id,
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
              "hidden group-hover/treeRootNodeTrigger:block": picturePath,
            })}
          />
        </button>

        {picturePath && (
          <div className="h-full w-full rounded group-hover/treeRootNodeTrigger:hidden">
            <img src={picturePath} className="h-full w-full" />
          </div>
        )}
      </span>

      {showNodeOrders && <div className="underline">{node.order}</div>}
      <NodeLabel label={node.name} searchInput={searchInput} />
    </div>
  );
};
