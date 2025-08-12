import { useEffect } from "react";

import { useStreamEnvironments } from "@/hooks/environment";
import { useWorkspaceListStore } from "@/store/workspaceList";

import { useMonitorWorkspacesList } from "./hooks/useMonitorWorkspacesList";
import { WorkspacesListItem } from "./WorkspacesListItem/WorkspacesListItem";

export const WorkspacesList = () => {
  const { environmentsSortedByOrder } = useStreamEnvironments();
  const { setActiveEnvironment } = useWorkspaceListStore();

  useEffect(() => {
    if (environmentsSortedByOrder) {
      setActiveEnvironment(environmentsSortedByOrder[0]);
    }
  }, [environmentsSortedByOrder, setActiveEnvironment]);

  useMonitorWorkspacesList();

  return (
    <div className="flex flex-col">
      {environmentsSortedByOrder?.map((environment) => (
        <WorkspacesListItem key={environment.id} environment={environment} />
      ))}
    </div>
  );
};
