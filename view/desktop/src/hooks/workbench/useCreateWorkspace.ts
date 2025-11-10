import {
  defaultBottomPanePanelState,
  defaultLayoutState,
  defaultSidebarPanelState,
  emptyGridState,
} from "@/constants/layoutPositions";
import { workspaceService } from "@/lib/services/workbench/workspaceService";
import {
  CreateWorkspaceInput,
  CreateWorkspaceOutput,
  DescribeAppOutput,
  ListWorkspacesOutput,
  WorkspaceInfo,
} from "@repo/window";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_DESCRIBE_APP_QUERY_KEY } from "../app";
import { useUpdateBottomPanel } from "../sharedStorage/layout/bottomPanel/useUpdateBottomPanel";
import { useUpdateSidebarPanel } from "../sharedStorage/layout/sidebar/useUpdateSidebarPanel";
import { useUpdateTabbedPane } from "../sharedStorage/layout/tabbedPane/useUpdateTabbedPane";
import { useUpdateLayout } from "../sharedStorage/layout/useUpdateLayout";
import { USE_LIST_WORKSPACES_QUERY_KEY } from "./useListWorkspaces";

export const USE_CREATE_WORKSPACE_MUTATION_KEY = "createWorkspace";

const createWorkspaceFn = async (input: CreateWorkspaceInput): Promise<CreateWorkspaceOutput> => {
  const result = await workspaceService.createWorkspace(input);

  if (result.status === "error") {
    throw new Error(String(result.error));
  }

  return result.data;
};

export const useCreateWorkspace = () => {
  const queryClient = useQueryClient();

  const { mutateAsync: updateTabbedPane } = useUpdateTabbedPane();
  const { mutateAsync: updateBottomPanel } = useUpdateBottomPanel();
  const { mutateAsync: updateSidebarPanel } = useUpdateSidebarPanel();

  const { mutateAsync: updateLayout } = useUpdateLayout();

  return useMutation<CreateWorkspaceOutput, Error, CreateWorkspaceInput>({
    mutationKey: [USE_CREATE_WORKSPACE_MUTATION_KEY],
    mutationFn: createWorkspaceFn,
    onSuccess: async (data, variables) => {
      const newWorkspace: WorkspaceInfo = {
        id: data.id,
        name: variables.name,
        lastOpenedAt: undefined,
      };

      await updateTabbedPane({ gridState: emptyGridState, workspaceId: newWorkspace.id });
      await updateBottomPanel({ ...defaultBottomPanePanelState, workspaceId: newWorkspace.id });
      await updateSidebarPanel({ ...defaultSidebarPanelState, workspaceId: newWorkspace.id });

      await updateLayout({ layout: defaultLayoutState, workspaceId: newWorkspace.id });

      queryClient.setQueryData<ListWorkspacesOutput>([USE_LIST_WORKSPACES_QUERY_KEY], (oldData) => {
        if (!oldData) return [newWorkspace];
        return [...oldData, newWorkspace];
      });

      if (data.active) {
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
