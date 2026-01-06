import { USE_STREAM_PROJECTS_QUERY_KEY, USE_STREAMED_ENVIRONMENTS_QUERY_KEY } from "@/adapters";
import { mainRouter } from "@/main/router/router";
import { mainWorkspaceService } from "@/main/services/mainWindowWorkspaceService";
import { OpenInTargetEnum } from "@/main/types";
import { MainWindow_OpenWorkspaceInput } from "@repo/ipc";
import { useMutation, useQueryClient } from "@tanstack/react-query";

import { USE_LIST_WORKSPACES_QUERY_KEY } from "../../adapters/tanstackQuery/workspace/useListWorkspaces";

export const USE_OPEN_WORKSPACE_QUERY_KEY = "openWorkspace";

export const useOpenWorkspace = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, MainWindow_OpenWorkspaceInput>({
    mutationKey: [USE_OPEN_WORKSPACE_QUERY_KEY],
    mutationFn: mainWorkspaceService.open,
    onSuccess: (_, { id, openInTarget }) => {
      if (openInTarget === OpenInTargetEnum.CURRENT_WINDOW) {
        mainRouter.navigate({ to: "/$workspaceId", params: { workspaceId: id } });

        queryClient.invalidateQueries({ queryKey: [USE_LIST_WORKSPACES_QUERY_KEY] });
        queryClient.invalidateQueries({ queryKey: [USE_STREAM_PROJECTS_QUERY_KEY] });
        queryClient.invalidateQueries({ queryKey: [USE_STREAMED_ENVIRONMENTS_QUERY_KEY] });
      }
    },
  });
};
