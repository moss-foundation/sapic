import { useGroupedEnvironments } from "../hooks/useGroupedEnvironments";
import { GroupedEnvironmentsListRoot } from "./GroupedEnvironmentsListRoot";

export const GroupedEnvironmentsList = () => {
  const { groupedEnvironments } = useGroupedEnvironments();

  return (
    <div>
      {groupedEnvironments?.map((groupedEnvironment) => (
        <GroupedEnvironmentsListRoot key={groupedEnvironment.projectId} groupedEnvironments={groupedEnvironment} />
      ))}
    </div>
  );
};
