import { useListWorkspaces } from "@/adapters/tanstackQuery/workspace";
import { useParams } from "@tanstack/react-router";

export const useCurrentWorkspace = () => {
  const { workspaceId } = useParams({ strict: false });
  const { data: workspaces } = useListWorkspaces();

  const currentWorkspace = workspaces?.find((workspace) => workspace.id === workspaceId);
  const currentWorkspaceId = workspaceId!;

  return {
    currentWorkspace,
    currentWorkspaceId,
  };
};
