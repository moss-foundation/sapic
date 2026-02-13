import { projectListStateService } from "@/workbench/domains/projectListItemState/service";
import { ProjectListItemState } from "@/workbench/domains/projectListItemState/types";
import { useQuery } from "@tanstack/react-query";

export const USE_GET_PROJECT_LIST_STATE_QUERY_KEY = "getProjectListState" as const;

export const useGetProjectListState = (workspaceId: string) => {
  return useQuery<ProjectListItemState, Error>({
    queryKey: [USE_GET_PROJECT_LIST_STATE_QUERY_KEY, workspaceId],
    queryFn: () => projectListStateService.get(workspaceId),
  });
};
