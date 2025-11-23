import { workspaceService } from "@/lib/services/workbench/workspaceService";
import { defaultLayoutState } from "@/workbench/domains/layout/defaults";
import { ListWorkspacesOutput, MainWindow_CreateWorkspaceInput, MainWindow_CreateWorkspaceOutput } from "@repo/ipc";
import { DescribeAppOutput, WorkspaceInfo } from "@repo/window";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_QUERY_KEY } from "../app";
import { useUpdateLayout } from "./layout/useUpdateLayout";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_CREATE_WORKSPACE_MUTATION_KEY = "createWorkspace";

const createWorkspaceFn = async (input: MainWindow_CreateWorkspaceInput): Promise<MainWindow_CreateWorkspaceOutput> => {
  const result = await workspaceService.createWorkspace(input);

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
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
