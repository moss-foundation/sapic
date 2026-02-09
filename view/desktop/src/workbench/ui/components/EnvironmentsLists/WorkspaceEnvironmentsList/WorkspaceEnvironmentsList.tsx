import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";

export const WorkspaceEnvironmentsList = () => {
  const { workspaceEnvironments } = useGetWorkspaceEnvironments();

  return (
    <ul>
      {workspaceEnvironments?.map((environment) => (
        <EnvironmentItem
          key={environment.id}
          environment={{
            id: environment.id,
            name: environment.name,
            totalVariables: environment.totalVariables,
            isActive: environment.isActive,
            order: environment.order,
            metadata: {
              isDirty: false,
            },
          }}
          type={ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE}
        />
      ))}
    </ul>
  );
};
