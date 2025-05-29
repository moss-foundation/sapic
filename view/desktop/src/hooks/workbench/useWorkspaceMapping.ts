import { useMemo } from "react";
import { WorkspaceInfo } from "@repo/moss-workbench";
import { useListWorkspaces } from "./useListWorkspaces";

export const useWorkspaceMapping = () => {
  const { data: workspaces } = useListWorkspaces();

  const { workspaceMap, getWorkspaceById, getWorkspaceByName } = useMemo(() => {
    if (!workspaces) {
      return {
        workspaceMap: new Map<string, WorkspaceInfo>(),
        getWorkspaceById: () => undefined,
        getWorkspaceByName: () => undefined,
      };
    }

    const workspaceMap = new Map<string, WorkspaceInfo>();

    workspaces.forEach((workspace) => {
      workspaceMap.set(workspace.id, workspace);
    });

    return {
      workspaceMap,
      getWorkspaceById: (id: string) => workspaceMap.get(id),
      getWorkspaceByName: (name: string) => workspaces.find((w) => w.displayName === name),
    };
  }, [workspaces]);

  return {
    workspaces: workspaces || [],
    workspaceMap,
    getWorkspaceById,
    getWorkspaceByName,
    // Legacy methods for backward compatibility
    getNameById: (id: string) => workspaceMap.get(id)?.displayName,
    getIdByName: (name: string) => {
      const workspace = workspaces?.find((w) => w.displayName === name);
      return workspace?.id;
    },
  };
};
