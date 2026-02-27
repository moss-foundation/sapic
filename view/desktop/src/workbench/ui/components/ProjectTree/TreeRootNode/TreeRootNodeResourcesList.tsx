import { useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useGetResourcesListItemState";
import { usePutResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/usePutResourcesListItemState";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";
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

  const shouldRenderChildNodes = expanded || isAddingRootFileNode || isAddingRootFolderNode;

  const { mutate: updateResourcesListState } = usePutResourcesListItemState();

  const handleExpand = () => {
    updateResourcesListState({
      resourcesListItemId: tree.id,
      expanded: !expanded,
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.Node>
      <Tree.NodeDetails
        className="flex cursor-pointer items-center gap-1 py-[5px]"
        style={{ paddingLeft: treePaddingLeft }}
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
          isAddingRootFileNode={isAddingRootFileNode}
          isAddingRootFolderNode={isAddingRootFolderNode}
          handleAddFormRootSubmit={handleRootAddFormSubmit}
          handleAddFormRootCancel={handleRootAddFormCancel}
        />
      )}
    </Tree.Node>
  );
};
