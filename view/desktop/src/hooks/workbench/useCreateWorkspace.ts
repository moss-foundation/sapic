import { mainWorkspaceService } from "@/main/services/mainWindowWorkspaceService";
import { defaultLayoutState } from "@/workbench/domains/layout/defaults";
import { WorkspaceInfo } from "@repo/base";
import { ListWorkspacesOutput, MainWindow_CreateWorkspaceInput, MainWindow_CreateWorkspaceOutput } from "@repo/ipc";
import { DescribeAppOutput } from "@repo/window";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACES_QUERY_KEY } from "../../adapters/tanstackQuery/workspace/useListWorkspaces";
import { USE_DESCRIBE_APP_QUERY_KEY } from "../app";
import { useUpdateLayout } from "./layout/useUpdateLayout";

export const USE_CREATE_WORKSPACE_MUTATION_KEY = "createWorkspace";

const createWorkspaceFn = async (input: MainWindow_CreateWorkspaceInput): Promise<MainWindow_CreateWorkspaceOutput> => {
  return await mainWorkspaceService.create(input);
};

export const useCreateWorkspace = () => {
  const queryClient = useQueryClient();

  const { mutateAsync: updateLayout } = useUpdateLayout();

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

      queryClient.setQueryData<ListWorkspacesOutput>([USE_LIST_WORKSPACES_QUERY_KEY], (oldData) => {
        if (!oldData) return [newWorkspace];
        return [...oldData, newWorkspace];
      });

      if (data.willReplace) {
        queryClient.setQueryData<DescribeAppOutput>([USE_DESCRIBE_APP_QUERY_KEY], (oldData) => {
          if (!oldData) return oldData;
          return {
            ...oldData,
            workspace: newWorkspace,
          };
        });
      }
    },
  });
};
