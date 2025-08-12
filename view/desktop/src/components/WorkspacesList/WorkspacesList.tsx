import { useEffect } from "react";

import { useStreamEnvironments } from "@/hooks/environment";
import { useWorkspaceListStore } from "@/store/workspaceList";

import { WorkspacesListItem } from "./WorkspacesListItem/WorkspacesListItem";

export const WorkspacesList = () => {
  const { data: environments } = useStreamEnvironments();
  const { setActiveEnvironment } = useWorkspaceListStore();

  useEffect(() => {
    if (environments) {
      setActiveEnvironment(environments[0]);
    }
  }, [environments, setActiveEnvironment]);

  return (
    <div className="flex flex-col">
      {environments?.map((environment) => (
        <WorkspacesListItem key={environment.id} environment={environment} />
      ))}
    </div>
  );
};
