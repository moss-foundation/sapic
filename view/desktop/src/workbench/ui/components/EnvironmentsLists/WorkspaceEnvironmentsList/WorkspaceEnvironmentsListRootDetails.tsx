import { useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { usePutEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/usePutEnvironmentListItemState";

import { ProjectTreeContext } from "../../ProjectTree/ProjectTreeContext";
import { WORKSPACE_ENVIRONMENTS_LIST_ID } from "../constants";

interface WorkspaceEnvironmentsListRootDetailsProps {
  expanded: boolean;
}

export const WorkspaceEnvironmentsListRootDetails = ({ expanded }: WorkspaceEnvironmentsListRootDetailsProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { showOrders } = useContext(ProjectTreeContext);

  const { mutate: updateEnvironmentListItemState } = usePutEnvironmentListItemState();

  const onHeaderClick = () => {
    if (expanded) return;

    updateEnvironmentListItemState({
      id: WORKSPACE_ENVIRONMENTS_LIST_ID,
      expanded: true,
      workspaceId: currentWorkspaceId,
    });
  };

  const onIconClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    updateEnvironmentListItemState({
      id: WORKSPACE_ENVIRONMENTS_LIST_ID,
      expanded: !expanded,
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.RootDetails>
      <Tree.RootTriggers className="overflow-hidden py-[2px]" onClick={onHeaderClick}>
        <button onClick={onIconClick} className="flex cursor-pointer items-center justify-center rounded-full">
          <Icon icon="ChevronRight" className={cn(expanded && "rotate-90")} />
        </button>

        {showOrders && <Tree.RootOrder order={0} />}
        <Tree.RootLabel label="Globals" />
      </Tree.RootTriggers>
    </Tree.RootDetails>
  );
};
