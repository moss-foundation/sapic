import { useStreamEnvironments } from "@/hooks";

import { GlobalEnvironmentsListItem } from "./GlobalEnvironmentsListItem/GlobalEnvironmentsListItem";
import { useMonitorGlobalEnvironmentsList } from "./hooks/useMonitorGlobalEnvironmentsList";

export const GlobalEnvironmentsList = () => {
  const { globalEnvironments } = useStreamEnvironments();

  useMonitorGlobalEnvironmentsList();

  return (
    <ul className="flex flex-col">
      {globalEnvironments?.map((environment) => (
        <GlobalEnvironmentsListItem key={environment.id} environment={environment} />
      ))}
    </ul>
  );
};
