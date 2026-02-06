import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";

import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { EnvironmentListType } from "../types";

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
          type={EnvironmentListType.GLOBAL}
        />
      ))}
    </ul>
  );
};
