import { useContext, useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useGetResourcesListItemState";
import { usePutResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/usePutResourcesListItemState";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";
import { useDropTargetResourcesList } from "./dnd/hooks/useDropTargetResourcesList";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";

interface TreeRootResourcesListProps {
  tree: ProjectTreeRootNode;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleRootAddFormSubmit: (name: string) => void;
  handleRootAddFormCancel: () => void;
}

export const TreeRootNodeResourcesList = ({
  tree,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleRootAddFormSubmit,
  handleRootAddFormCancel,
}: TreeRootResourcesListProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { treePaddingLeft } = useContext(ProjectTreeContext);
  const { data: expanded } = useGetResourcesListItemState(tree.id, currentWorkspaceId);

  const ref = useRef<HTMLDivElement | null>(null);
  const shouldRenderChildNodes = expanded || isAddingRootFileNode || isAddingRootFolderNode;

  const { mutate: updateResourcesListState } = usePutResourcesListItemState();
  const { instruction } = useDropTargetResourcesList({ ref, tree });

  const handleExpand = () => {
    updateResourcesListState({
      resourcesListItemId: tree.id,
      expanded: !expanded,
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.Node instruction={instruction}>
      <Tree.NodeDetails
        ref={ref}
        className="flex cursor-pointer items-center gap-1 py-[5px]"
        style={{ paddingLeft: treePaddingLeft * 2 }}
      >
        <Tree.RootNodeTriggers onClick={handleExpand}>
          <Icon icon="ChevronRight" className={cn(shouldRenderChildNodes && "rotate-90")} />
          <div className="flex items-center gap-1">
            <Tree.RootNodeLabel label="Resources" className="text-sm" />
            {/* <Tree.NodeDirCount count={123} /> */}
          </div>
        </Tree.RootNodeTriggers>
      </Tree.NodeDetails>

      {shouldRenderChildNodes && (
        <TreeRootNodeChildren
          node={tree}
          offsetLeft={treePaddingLeft * 2}
          depth={2}
          isAddingRootFileNode={isAddingRootFileNode}
          isAddingRootFolderNode={isAddingRootFolderNode}
          handleAddFormRootSubmit={handleRootAddFormSubmit}
          handleAddFormRootCancel={handleRootAddFormCancel}
        />
      )}
    </Tree.Node>
  );
};
