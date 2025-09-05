import { GroupedEnvironmentsListContext } from "./GroupedEnvironmentsListContext";
import { GroupedEnvironmentsListRoot } from "./GroupedEnvironmentsListRoot/GroupedEnvironmentsListRoot";
import { useGroupedWithEnvironments } from "./hooks/useGroupedWithEnvironments";

export const GroupedEnvironmentsList = () => {
  const { groupedWithEnvironments } = useGroupedWithEnvironments();

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
