import { useMemo } from "react";

import { WorkspaceInfo } from "@repo/base";

import { useListWorkspaces } from "./useListWorkspaces";

export const useWorkspaceMapping = () => {
  const { data: workspaces } = useListWorkspaces();

  const { workspaceMap, getWorkspaceById } = useMemo(() => {
    if (!workspaces) {
      return {
        workspaceMap: new Map<string, WorkspaceInfo>(),
        getWorkspaceById: () => undefined,
      };
    }

    const workspaceMap = new Map<string, WorkspaceInfo>();

    workspaces.forEach((workspace) => {
      workspaceMap.set(workspace.id, workspace);
    });

    return {
      workspaceMap,
      getWorkspaceById: (id: string) => workspaceMap.get(id),
    };
  }, [workspaces]);

  return {
    workspaces: workspaces || [],
    workspaceMap,
    getWorkspaceById,
    getNameById: (id: string) => workspaceMap.get(id)?.name,
  };
};
