import { useDeleteEnvironment, useStreamEnvironments } from "@/hooks";
import { useBatchUpdateEnvironment } from "@/hooks/workspace/environment/useBatchUpdateEnvironment";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { EnvironmentListType } from "../EnvironmentItem/types";
import { useGroupedEnvironments } from "../hooks/useGroupedEnvironments";

interface UseDeleteEnvironmentItemProps {
  environment: StreamEnvironmentsEvent;
  type: EnvironmentListType;
}

export const useDeleteEnvironmentItem = ({ environment, type }: UseDeleteEnvironmentItemProps) => {
  const { globalEnvironments } = useStreamEnvironments();
  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: batchUpdateEnvironment } = useBatchUpdateEnvironment();
  const { groupedEnvironments } = useGroupedEnvironments();

  const handleDeleteEnvironment = async () => {
    console.log("handleDeleteEnvironment", environment, type);
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

      const collectionEnvironments = groupedEnvironments.find(
        (group) => group.collectionId === environment.collectionId
      )?.environments;

      const environmentsAfterDeleted = collectionEnvironments?.filter((env) => env.order! > environment.order!);

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
