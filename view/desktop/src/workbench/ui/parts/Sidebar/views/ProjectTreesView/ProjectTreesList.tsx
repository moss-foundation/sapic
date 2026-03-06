import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetProjectListState } from "@/workbench/adapters/tanstackQuery/projectListState/useGetProjectListState";
import { usePutProjectListState } from "@/workbench/adapters/tanstackQuery/projectListState/usePutProjectListState";
import { useWorkspaceModeStore } from "@/workbench/store/workspaceMode";
import { ProjectTree } from "@/workbench/ui/components";
import { useMonitorEnvironmentsLists } from "@/workbench/ui/components/EnvironmentsLists/dnd/hooks/useMonitorEnvironmentsLists";
import { useProjectsTrees } from "@/workbench/ui/components/ProjectTree/hooks/useProjectsTrees";

export const ProjectTreesList = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { projectsTreesSortedByOrder, isLoading } = useProjectsTrees();

  const { displayMode } = useWorkspaceModeStore();

  const { data: expanded } = useGetProjectListState(currentWorkspaceId);
  useMonitorEnvironmentsLists();

  return (
    <div>
      <ProjectTreesHeader />

      {expanded && (
        <div className="flex h-full flex-col">
          {!isLoading &&
            projectsTreesSortedByOrder.map((tree) => (
              <ProjectTree key={tree.id} tree={tree} displayMode={displayMode} />
            ))}
        </div>
      )}
    </div>
  );
};

export const ProjectTreesHeader = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { mutate: updateProjectListState } = usePutProjectListState();
  const { data: expanded } = useGetProjectListState(currentWorkspaceId);

  const handleToggleProjectList = () => {
    updateProjectListState({
      expanded: !expanded,
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.RootNodeDetails>
      <Tree.RootNodeTriggers
        onClick={handleToggleProjectList}
        className="flex cursor-pointer items-center gap-1 py-[5px] pl-2"
      >
        <Icon icon="ChevronRight" className={cn(expanded && "rotate-90")} />
        <Tree.RootNodeLabel className="text-(--moss-secondary-foreground) text-sm" label="Projects" />
      </Tree.RootNodeTriggers>
    </Tree.RootNodeDetails>
  );
};
