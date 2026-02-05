import { useStreamProjects } from "@/adapters";
import { useCurrentWorkspace } from "@/hooks";
import { useBatchGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useBatchGetEnvironmentListItemState";

export const useGetProjectEnvironmentListItems = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { data: projects } = useStreamProjects();

  const { data: environmentListItemStates } = useBatchGetEnvironmentListItemState(
    projects?.map((project) => project.id) ?? [],
    currentWorkspaceId
  );

  return (
    projects?.map((project) => {
      const projectEnvironments = environmentListItemStates?.filter((state) => state.id === project.id);
      return {
        ...project,
        expanded: projectEnvironments?.some((state) => state.expanded) ?? false,
      };
    }) ?? []
  );
};
