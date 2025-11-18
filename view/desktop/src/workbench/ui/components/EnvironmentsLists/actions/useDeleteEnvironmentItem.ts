import { useDeleteEnvironment, useStreamEnvironments } from "@/adapters/tanstackQuery/environment";
import { useBatchUpdateEnvironment } from "@/adapters/tanstackQuery/environment/useBatchUpdateEnvironment";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { useGroupedEnvironments } from "../hooks/useGroupedEnvironments";
import { EnvironmentListType } from "../types";

interface UseDeleteEnvironmentItemProps {
  environment: StreamEnvironmentsEvent;
  type: EnvironmentListType;
}

export const useDeleteEnvironmentItem = ({ environment, type }: UseDeleteEnvironmentItemProps) => {
  const { globalEnvironments } = useStreamEnvironments();
  const { groupedEnvironments } = useGroupedEnvironments();

  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: batchUpdateEnvironment } = useBatchUpdateEnvironment();

  const handleDeleteEnvironment = async () => {
    if (type === "GlobalEnvironmentItem") {
      await deleteEnvironment({ id: environment.id });

      const environmentsAfterDeleted = globalEnvironments?.filter((env) => env.order! > environment.order!);

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

    if (type === "GroupedEnvironmentItem") {
      await deleteEnvironment({ id: environment.id });

      const projectEnvironments = groupedEnvironments.find(
        (group) => group.projectId === environment.projectId
      )?.environments;

      const environmentsAfterDeleted = projectEnvironments?.filter((env) => env.order! > environment.order!);

      if (environmentsAfterDeleted) {
        await batchUpdateEnvironment({
          items: environmentsAfterDeleted?.map((env) => ({
            id: env.id,
            order: env.order! - 1,
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
