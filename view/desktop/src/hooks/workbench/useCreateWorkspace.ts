import { mainWorkspaceService } from "@/main/services/mainWindowWorkspaceService";
import { useUpdateLayout } from "@/workbench/adapters";
import { defaultLayoutState } from "@/workbench/domains/layout/defaults";
import { WorkspaceInfo } from "@repo/base";
import { ListWorkspacesOutput, MainWindow_CreateWorkspaceInput, MainWindow_CreateWorkspaceOutput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { useBatchPutActivityBarItemState } from "@/workbench/adapters/tanstackQuery/activityBarItemState/useBatchPutActivityBarItemState";
import { defaultStates } from "@/workbench/domains/activityBarItemState/defaults";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "../../adapters/tanstackQuery/workspace/useListWorkspaces";

export const USE_CREATE_WORKSPACE_MUTATION_KEY = "createWorkspace";

const createWorkspaceFn = async (input: MainWindow_CreateWorkspaceInput): Promise<MainWindow_CreateWorkspaceOutput> => {
  return await mainWorkspaceService.create(input);
};

export const useCreateWorkspace = () => {
  const queryClient = useQueryClient();

  const { mutateAsync: updateLayout } = useUpdateLayout();
  const { mutateAsync: batchPutActivityBarItemState } = useBatchPutActivityBarItemState();

  return useMutation<MainWindow_CreateWorkspaceOutput, Error, MainWindow_CreateWorkspaceInput>({
    mutationKey: [USE_CREATE_WORKSPACE_MUTATION_KEY],
    mutationFn: createWorkspaceFn,
    onSuccess: async (data, variables) => {
      const newWorkspace: WorkspaceInfo = {
        id: data.id,
        name: variables.name,
        lastOpenedAt: undefined,
      };

      await updateLayout({ layout: defaultLayoutState, workspaceId: newWorkspace.id });
      await batchPutActivityBarItemState({
        activityBarItemStates: defaultStates.map((state) => ({ ...state, workspaceId: newWorkspace.id })),
        workspaceId: newWorkspace.id,
      });

      queryClient.setQueryData<ListWorkspacesOutput>([USE_LIST_WORKSPACES_QUERY_KEY], (oldData) => {
        if (!oldData) return [newWorkspace];
        return [...oldData, newWorkspace];
      });
    },
  });
};
