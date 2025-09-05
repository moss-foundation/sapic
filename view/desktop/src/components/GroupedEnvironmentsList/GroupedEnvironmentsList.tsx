import { GroupedEnvironmentsListContext } from "./GroupedEnvironmentsListContext";
import { GroupedEnvironmentsListRoot } from "./GroupedEnvironmentsListRoot/GroupedEnvironmentsListRoot";
import { useGroupedWithEnvironments } from "./hooks/useGroupedWithEnvironments";
import { useMonitorGroupedEnvironments } from "./hooks/useMonitorGroupedEnvironments";

export const GroupedEnvironmentsList = () => {
  const { groupedWithEnvironments } = useGroupedWithEnvironments();

  useMonitorGroupedEnvironments();

  return (
    <div className="flex flex-col">
      {groupedWithEnvironments?.map((groupedWithEnv) => (
        <GroupedEnvironmentsListContext.Provider
          key={groupedWithEnv.collectionId}
          value={{
            id: groupedWithEnv.collectionId,
            name: "",
            order: groupedWithEnv.order ?? 0,
            treePaddingLeft: 0,
            treePaddingRight: 0,
            nodeOffset: 0,
            showOrders: false,
          }}
        >
          <GroupedEnvironmentsListRoot key={groupedWithEnv.collectionId} groupedWithEnvironments={groupedWithEnv} />
        </GroupedEnvironmentsListContext.Provider>
      ))}
    </div>
  );
};
