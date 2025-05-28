import { useMemo } from "react";
import { useListWorkspaces } from "./useListWorkspaces";

export const useWorkspaceMapping = () => {
  const { data: workspaces } = useListWorkspaces();

  const { idToName, nameToId, getWorkspaceById, getWorkspaceByName } = useMemo(() => {
    if (!workspaces) {
      return {
        idToName: new Map<string, string>(),
        nameToId: new Map<string, string>(),
        getWorkspaceById: () => undefined,
        getWorkspaceByName: () => undefined,
      };
    }

    const idToName = new Map<string, string>();
    const nameToId = new Map<string, string>();

    workspaces.forEach((workspace) => {
      idToName.set(workspace.id, workspace.displayName);
      nameToId.set(workspace.displayName, workspace.id);
    });

    return {
      idToName,
      nameToId,
      getWorkspaceById: (id: string) => workspaces.find((w) => w.id === id),
      getWorkspaceByName: (name: string) => workspaces.find((w) => w.displayName === name),
    };
  }, [workspaces]);

  return {
    workspaces: workspaces || [],
    idToName,
    nameToId,
    getWorkspaceById,
    getWorkspaceByName,
    getNameById: (id: string) => idToName.get(id),
    getIdByName: (name: string) => nameToId.get(name),
  };
};
