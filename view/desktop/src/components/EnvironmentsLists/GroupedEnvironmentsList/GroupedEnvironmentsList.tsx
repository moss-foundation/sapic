import { useGroupedEnvironments } from "../hooks/useGroupedEnvironments";
import { GroupedEnvironmentsListRoot } from "./GroupedEnvironmentsListRoot";

export const GroupedEnvironmentsList = () => {
  const { groupedEnvironments } = useGroupedEnvironments();

  return (
    <div className="flex flex-col">
      {groupedEnvironments?.map((groupedEnv) => (
        <GroupedEnvironmentsListRoot key={groupedEnv.collectionId} groupedEnvironments={groupedEnv} />
      ))}
    </div>
  );
};
