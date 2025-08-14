import { useEffect } from "react";

import { useStreamEnvironments } from "@/hooks/environment";
import { useWorkspaceListStore } from "@/store/workspaceList";

import { GlobalEnvironmentsListItem } from "./GlobalEnvironmentsListItem/GlobalEnvironmentsListItem";
import { useMonitorGlobalEnvironmentsList } from "./hooks/useMonitorGlobalEnvironmentsList";

export const GlobalEnvironmentsList = () => {
  const { globalEnvironments } = useStreamEnvironments();
  const { setActiveEnvironment } = useWorkspaceListStore();

  //TODO this is a temporary solution, just for demonstration purposes
  useEffect(() => {
    if (globalEnvironments) {
      setActiveEnvironment(globalEnvironments[0]);
    }
  }, [globalEnvironments, setActiveEnvironment]);

  useMonitorGlobalEnvironmentsList();

  return (
    <div className="flex flex-col">
      {globalEnvironments?.map((environment) => (
        <GlobalEnvironmentsListItem key={environment.id} environment={environment} />
      ))}
    </div>
  );
};
