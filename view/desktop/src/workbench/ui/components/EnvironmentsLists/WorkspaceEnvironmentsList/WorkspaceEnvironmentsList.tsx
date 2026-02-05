import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";

import { EnvironmentListItem } from "../EnvironmentItem/EnvironmentListItem";
import { EnvironmentListType } from "../types";

export const WorkspaceEnvironmentsList = () => {
  const { sortedWorkspaceEnvironmentsByOrder } = useGetWorkspaceEnvironments();

  return (
    <ul>
      {sortedWorkspaceEnvironmentsByOrder?.map((environment) => (
        <EnvironmentListItem
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
          type={EnvironmentListType.GLOBAL}
        />
      ))}
    </ul>
  );
};
