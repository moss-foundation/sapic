import { useDeleteEnvironment, useStreamEnvironments } from "@/adapters/tanstackQuery/environment";
import { useAllStreamedProjectEnvironments } from "@/adapters/tanstackQuery/environment/derived/useAllStreamedProjectEnvironments";
import { useBatchUpdateEnvironment } from "@/adapters/tanstackQuery/environment/useBatchUpdateEnvironment";
import { StreamEnvironmentsEvent } from "@repo/ipc";

import { EnvironmentListType } from "../types";

interface UseDeleteEnvironmentItemProps {
  environment: StreamEnvironmentsEvent;
  type: EnvironmentListType;
}

export const useDeleteEnvironmentItem = ({ environment, type }: UseDeleteEnvironmentItemProps) => {
  const { data: workspaceEnvironments } = useStreamEnvironments();
  const { allProjectEnvironments } = useAllStreamedProjectEnvironments();

  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: batchUpdateEnvironment } = useBatchUpdateEnvironment();

  const handleDeleteEnvironment = async () => {
    if (type === "GlobalEnvironmentItem") {
      await deleteEnvironment({ id: environment.id });

      const environmentsAfterDeleted = workspaceEnvironments?.filter((env) => env.order! > environment.order!);

      if (environmentsAfterDeleted) {
        await batchUpdateEnvironment({
          items: environmentsAfterDeleted.map((env) => ({
            id: env.id,
            order: env.order! - 1,
            varsToAdd: [],
            varsToUpdate: [],
            varsToDelete: [],
          })),
        });
      }
    }

    if (type === "GroupedEnvironmentItem") {
      await deleteEnvironment({ id: environment.id, projectId: environment.projectId });

      const environmentsAfterDeleted = allProjectEnvironments?.filter(
        (env) => (env.order ?? 0) > (environment.order ?? 0)
      );

      if (environmentsAfterDeleted) {
        await batchUpdateEnvironment({
          items: environmentsAfterDeleted?.map((env) => ({
            id: env.id,
            order: (env.order ?? 0) - 1,
            varsToAdd: [],
            varsToUpdate: [],
            varsToDelete: [],
          })),
        });
      }
    }
  };

  return {
    handleDeleteEnvironment,
  };
};
