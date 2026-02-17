import { useContext } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { usePutEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/usePutEnvironmentListItemState";
import { ListProjectItem } from "@repo/ipc";

import { ProjectTreeContext } from "../../ProjectTree/ProjectTreeContext";

interface ProjectEnvironmentsListRootControlsProps {
  project: ListProjectItem;
  expanded: boolean;
  count: number;
}

export const ProjectEnvironmentsListRootControls = ({
  project,
  expanded,
  count,
}: ProjectEnvironmentsListRootControlsProps) => {
  const { treePaddingLeft } = useContext(ProjectTreeContext);
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutate: updateEnvironmentListItemState } = usePutEnvironmentListItemState();

  const onHeaderClick = () => {
    if (expanded) return;

    updateEnvironmentListItemState({
      id: project.id,
      expanded: true,
      workspaceId: currentWorkspaceId,
    });
  };

  const onIconClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    updateEnvironmentListItemState({
      id: project.id,
      expanded: !expanded,
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.RootNodeControls className="cursor-pointer text-sm">
      <Tree.RootNodeTriggers
        className="overflow-hidden py-[2px]"
        onClick={onHeaderClick}
        style={{ paddingLeft: treePaddingLeft }}
      >
        <button onClick={onIconClick} className="flex cursor-pointer items-center justify-center rounded-full">
          <Icon icon="ChevronRight" className={cn(expanded && "rotate-90")} />
        </button>

        <div className="flex items-center gap-1">
          <Tree.RootNodeLabel label="Environments" className="text-sm" />
          <Tree.NodeDirCount count={count} />
        </div>
      </Tree.RootNodeTriggers>
    </Tree.RootNodeControls>
  );
};
