import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { usePutEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/usePutEnvironmentListItemState";
import { StreamProjectsEvent } from "@repo/ipc";

interface ProjectEnvironmentsListRootControlsProps {
  project: StreamProjectsEvent;
  expanded: boolean;
}

export const ProjectEnvironmentsListRootControls = ({
  project,
  expanded,
}: ProjectEnvironmentsListRootControlsProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutate: updateEnvironmentListItemState } = usePutEnvironmentListItemState();

  const onHeaderClick = () => {
    if (expanded) return;

    updateEnvironmentListItemState({
      environmentListItemState: {
        id: project.id,
        expanded: true,
      },
      workspaceId: currentWorkspaceId,
    });
  };

  const onIconClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    updateEnvironmentListItemState({
      environmentListItemState: {
        id: project.id,
        expanded: !expanded,
      },
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.RootNodeControls>
      {/*TODO: remove this comment if it isn't removed 34px height === py-[9px] */}
      <Tree.RootNodeTriggers className="overflow-hidden py-[2px]" onClick={onHeaderClick}>
        <button
          onClick={onIconClick}
          className="hover:background-(--moss-list-background-hover) flex size-4 cursor-pointer items-center justify-center rounded-full"
        >
          <Icon icon="ChevronRight" className={cn(expanded && "rotate-90")} />
        </button>

        <Tree.RootNodeLabel label={project.name} />
      </Tree.RootNodeTriggers>
    </Tree.RootNodeControls>
  );
};
