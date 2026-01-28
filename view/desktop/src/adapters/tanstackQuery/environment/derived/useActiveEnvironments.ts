import { useGroupedEnvironments } from "@/workbench/ui/components/EnvironmentsLists/hooks/useGroupedEnvironments";

import { useStreamEnvironments } from "../useStreamEnvironments";

export const useActiveEnvironments = () => {
  const { data: workspaceEnvironments } = useStreamEnvironments();
  const { groupedEnvironments } = useGroupedEnvironments();

  const activeWorkspaceEnvironment = workspaceEnvironments?.find((environment) => environment.isActive);
  const activeProjectEnvironments = groupedEnvironments?.flatMap((group) =>
    group.environments.filter((environment) => environment.isActive)
  );

  return {
    activeWorkspaceEnvironment,
    activeProjectEnvironments,
  };
};
