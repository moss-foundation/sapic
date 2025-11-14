import { useStreamProjects } from "@/hooks";
import { useUpdateEnvironmentGroup } from "@/hooks/workspace/environment/useUpdateEnvironmentGroup";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { GroupedEnvironments } from "../types";
import { GroupedEnvironmentsListRootActions } from "./GroupedEnvironmentsListRootActions";

interface GroupedEnvironmentsListRootControlsProps {
  groupedEnvironments: GroupedEnvironments;
}

export const GroupedEnvironmentsListRootControls = ({
  groupedEnvironments,
}: GroupedEnvironmentsListRootControlsProps) => {
  const { data: projects } = useStreamProjects();

  const { mutate: updateEnvironmentGroup } = useUpdateEnvironmentGroup();

  const projectName = projects?.find((project) => project.id === groupedEnvironments.projectId)?.name;

  const onHeaderClick = () => {
    if (!groupedEnvironments.expanded) {
      updateEnvironmentGroup({
        projectId: groupedEnvironments.projectId,
        expanded: true,
      });
    }
  };

  const onIconClick = () => {
    updateEnvironmentGroup({
      projectId: groupedEnvironments.projectId,
      expanded: !groupedEnvironments.expanded,
    });
  };

  return (
    <Tree.RootNodeControls>
      <Tree.RootNodeTriggers className="overflow-hidden" onClick={onHeaderClick}>
        <button
          onClick={onIconClick}
          className="hover:background-(--moss-list-background-hover) flex size-4 cursor-pointer items-center justify-center rounded-full"
        >
          <Icon icon="ChevronRight" className={cn(groupedEnvironments.expanded && "rotate-90")} />
        </button>

        <Tree.RootNodeLabel label={projectName ?? ""} />
      </Tree.RootNodeTriggers>

      <Tree.RootNodeActions>
        <GroupedEnvironmentsListRootActions />
      </Tree.RootNodeActions>
    </Tree.RootNodeControls>
  );
};
