import { useGroupedEnvironments } from "../hooks/useGroupedEnvironments";
import { GroupedEnvironmentsListRoot } from "./GroupedEnvironmentsListRoot";

export const GroupedEnvironmentsList = () => {
  const { groupedEnvironments } = useGroupedEnvironments();

  return (
    <div>
      {groupedEnvironments?.map((groupedEnv) => (
        <GroupedEnvironmentsListRoot key={groupedEnv.projectId} groupedEnvironments={groupedEnv} />
      ))}
    </div>
  );
};
