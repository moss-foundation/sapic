import { useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
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

  const { data: workspaceListState } = useGetWorkspaceListState(currentWorkspaceId);
  const expanded = workspaceListState?.expanded ?? false;

  const { mutate: updateWorkspaceListState } = usePutWorkspaceListState();

  const handleExpand = () => {
    updateWorkspaceListState({
      workspaceListState: { expanded: !expanded },
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.Node>
      <Tree.NodeControls
        className="hover:background-(--moss-list-background-hover) flex cursor-pointer items-center gap-1 py-[5px]"
        style={{ paddingLeft: treePaddingLeft }}
      >
        <Tree.RootNodeTriggers onClick={handleExpand}>
          <Icon icon="ChevronRight" className={cn(expanded && "rotate-90")} />
          <div className="flex items-center gap-1">
            <Tree.RootNodeLabel label="Resources" />
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
