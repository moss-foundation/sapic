import { useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetResourcesListState } from "@/workbench/adapters/tanstackQuery/resourcesListState/useGetResourcesListState";
import { usePutResourcesListState } from "@/workbench/adapters/tanstackQuery/resourcesListState/usePutResourcesListState";
import { useGetWorkspaceListState } from "@/workbench/adapters/tanstackQuery/workspaceListState/useGetWorkspaceListState";
import { usePutWorkspaceListState } from "@/workbench/adapters/tanstackQuery/workspaceListState/usePutWorkspaceListItemState";

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

export const TreeRootResourcesList = ({
  tree,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleRootAddFormSubmit,
  handleRootAddFormCancel,
}: TreeRootResourcesListProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { treePaddingLeft } = useContext(ProjectTreeContext);

  const { data: resourcesListState } = useGetResourcesListState(tree.id, currentWorkspaceId);
  const expanded = resourcesListState?.expanded ?? false;

  const { mutate: updateResourcesListState } = usePutResourcesListState();

  const handleExpand = () => {
    updateResourcesListState({
      resourcesListItemId: tree.id,
      resourcesListItemState: { expanded: !expanded },
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.Node>
      <Tree.NodeControls
        className="flex cursor-pointer items-center gap-1 py-[5px]"
        style={{ paddingLeft: treePaddingLeft }}
      >
        <Tree.RootNodeTriggers onClick={handleExpand}>
          <Icon icon="ChevronRight" className={cn(expanded && "rotate-90")} />
          <div className="flex items-center gap-1">
            <Tree.RootNodeLabel label="Resources" className="text-sm" />
            {/* <Tree.NodeDirCount count={123} /> */}
          </div>
        </Tree.RootNodeTriggers>
      </Tree.NodeControls>

      {expanded && (
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
