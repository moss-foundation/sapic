import { useDeleteEnvironment } from "@/adapters/tanstackQuery/environment";
import { useGetProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironments";
import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { useCurrentWorkspace } from "@/hooks/workspace/derived/useCurrentWorkspace";
import { useBatchPutEnvironmentItemState } from "@/workbench/adapters/tanstackQuery/environmentItemState/useBatchPutEnvironmentItemState";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";

interface UseDeleteEnvironmentItemProps {
  environment: EnvironmentSummary;
  type: ENVIRONMENT_ITEM_DRAG_TYPE;
}

export const useDeleteEnvironmentItem = ({ environment, type }: UseDeleteEnvironmentItemProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { workspaceEnvironments } = useGetWorkspaceEnvironments();
  const { projectEnvironments } = useGetProjectEnvironments(environment.projectId);

  const { mutateAsync: deleteEnvironment } = useDeleteEnvironment();
  const { mutateAsync: batchPutEnvironmentItemState } = useBatchPutEnvironmentItemState();

  const handleDeleteEnvironment = async () => {
    if (type === ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE) {
      await deleteEnvironment({ id: environment.id });

      const environmentsAfterDeleted = workspaceEnvironments?.filter((env) => env.order! > environment.order!);

      if (!environmentsAfterDeleted || environmentsAfterDeleted.length === 0) return;

      await batchPutEnvironmentItemState({
        environmentItemStates: environmentsAfterDeleted.map((env) => ({
          id: env.id,
          order: (env.order ?? 0) - 1,
        })),
        workspaceId: currentWorkspaceId,
      });
    }

    if (type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT) {
      await deleteEnvironment({ id: environment.id, projectId: environment.projectId ?? undefined });

      const environmentsAfterDeleted = projectEnvironments?.filter((env) => {
        return env.order && environment.order && env.order > environment.order;
      });

      if (!environmentsAfterDeleted || environmentsAfterDeleted.length === 0) return;

      if (environmentsAfterDeleted) {
        await batchPutEnvironmentItemState({
          environmentItemStates: environmentsAfterDeleted.map((env) => ({
            id: env.id,
            order: (env.order ?? 0) - 1,
          })),
          workspaceId: currentWorkspaceId,
        });
      }
    }
  };

  return {
    handleDeleteEnvironment,
  };
};
