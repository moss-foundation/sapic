import { useStreamEnvironments } from "../useStreamEnvironments";

export const useActiveEnvironments = () => {
  const { globalEnvironments, projectEnvironments } = useStreamEnvironments();

  const activeGlobalEnvironment = globalEnvironments.find((environment) => environment.isActive);
  const activeProjectEnvironments = projectEnvironments.filter((environment) => environment.isActive);

  return {
    activeGlobalEnvironment,
    activeProjectEnvironments,
  };
};
