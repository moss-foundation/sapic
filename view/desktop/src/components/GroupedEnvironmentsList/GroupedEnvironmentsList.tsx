import { GroupedEnvironmentsListRoot } from "./GroupedEnvironmentsListRoot/GroupedEnvironmentsListRoot";
import { useGroupedWithEnvironments } from "./hooks/useGroupedWithEnvironments";

export const GroupedEnvironmentsList = () => {
  const { groupedWithEnvironments } = useGroupedWithEnvironments();

  return (
    <ul className="flex flex-col">
      {groupedWithEnvironments?.map((groupedWithEnvironments) => (
        <GroupedEnvironmentsListRoot
          key={groupedWithEnvironments.collectionId}
          groupedWithEnvironments={groupedWithEnvironments}
        />
      ))}
    </ul>
  );
};
