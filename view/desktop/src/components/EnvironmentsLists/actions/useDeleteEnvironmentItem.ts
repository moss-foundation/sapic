import { useDeleteEnvironment, useStreamEnvironments } from "@/hooks";
import { useBatchUpdateEnvironment } from "@/hooks/workspace/environment/useBatchUpdateEnvironment";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { EnvironmentListType } from "../EnvironmentItem/types";

interface UseDeleteEnvironmentItemProps {
  environment: StreamEnvironmentsEvent;
  type: EnvironmentListType;
}

export const useDeleteEnvironmentItem = ({ environment, type }: UseDeleteEnvironmentItemProps) => {
  const { globalEnvironments } = useStreamEnvironments();
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
  };

  return {
    handleDeleteEnvironment,
  };
};
