import { useProjectsTrees } from "@/adapters/tanstackQuery/project/derivedHooks/useProjectsTrees";
import { useCurrentWorkspace } from "@/hooks";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetProjectListState } from "@/workbench/adapters/tanstackQuery/projectListItemState/useGetProjectListState";
import { usePutProjectListState } from "@/workbench/adapters/tanstackQuery/projectListItemState/usePutProjectListState";
import { useWorkspaceModeStore } from "@/workbench/store/workspaceMode";
import { ProjectTree } from "@/workbench/ui/components";

export const ProjectTreesList = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { projectsTreesSortedByOrder, isLoading } = useProjectsTrees();

  const { displayMode } = useWorkspaceModeStore();

  const { data: projectListState } = useGetProjectListState(currentWorkspaceId);

  return (
    <div>
      <ProjectTreesHeader />

      {projectListState?.expanded && (
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
  const { data: projectListState } = useGetProjectListState(currentWorkspaceId);

  const handleToggleProjectList = () => {
    updateProjectListState({
      projectListState: { expanded: !projectListState?.expanded },
      workspaceId: currentWorkspaceId,
    });
  };

  return (
    <Tree.RootNodeControls>
      <Tree.RootNodeTriggers
        onClick={handleToggleProjectList}
        className="flex cursor-pointer items-center gap-1 py-[5px] pl-2"
      >
        <Icon icon="ChevronRight" className={cn(projectListState?.expanded && "rotate-90")} />
        <Tree.RootNodeLabel className="text-(--moss-gray-6) text-sm" label="Projects" />
      </Tree.RootNodeTriggers>
    </Tree.RootNodeControls>
  );
};
